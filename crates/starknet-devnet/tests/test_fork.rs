pub mod common;

mod fork_tests {
    use std::str::FromStr;
    use std::sync::Arc;

    use hyper::Body;
    use serde_json::json;
    use starknet_core::constants::CAIRO_0_ACCOUNT_CONTRACT_HASH;
    use starknet_rs_accounts::{
        Account, AccountFactory, AccountFactoryError, Call, ExecutionEncoding,
        OpenZeppelinAccountFactory, SingleOwnerAccount,
    };
    use starknet_rs_contract::ContractFactory;
    use starknet_rs_core::chain_id;
    use starknet_rs_core::types::contract::legacy::LegacyContractClass;
    use starknet_rs_core::types::{
        BlockId, BlockTag, ContractClass, FieldElement, FunctionCall,
        MaybePendingBlockWithTxHashes, StarknetError,
    };
    use starknet_rs_core::utils::{get_selector_from_name, get_udc_deployed_address};
    use starknet_rs_providers::{Provider, ProviderError};

    use crate::common::background_devnet::BackgroundDevnet;
    use crate::common::utils::{
        assert_cairo1_classes_equal, assert_tx_successful, get_json_body,
        get_simple_contract_in_sierra_and_compiled_class_hash, resolve_path,
    };

    const SEPOLIA_URL: &str = "http://rpc.pathfinder.equilibrium.co/integration-sepolia/rpc/v0_7";
    const SEPOLIA_GENESIS_BLOCK_HASH: &str =
        "0x19f675d3fb226821493a6ab9a1955e384bba80f130de625621a418e9a7c0ca3";

    #[tokio::test]
    async fn test_fork_status() {
        let origin_devnet =
            BackgroundDevnet::spawn_with_additional_args(&["--state-archive-capacity", "full"])
                .await
                .unwrap();

        let origin_status =
            get_json_body(origin_devnet.get("/fork_status", None).await.unwrap()).await;
        assert_eq!(origin_status, serde_json::json!({}));

        origin_devnet
            .post_json("/create_block".into(), Body::from(json!({}).to_string()))
            .await
            .unwrap();
        let fork_devnet = origin_devnet.fork().await.unwrap();

        let fork_status = get_json_body(fork_devnet.get("/fork_status", None).await.unwrap()).await;
        assert_eq!(
            url::Url::from_str(fork_status["url"].as_str().unwrap()).unwrap(),
            url::Url::from_str(&origin_devnet.url).unwrap()
        );
        assert_eq!(fork_status["block"].as_i64().unwrap(), 0);
    }

    #[tokio::test]
    async fn test_forking_sepolia_genesis_block() {
        let fork_devnet =
            BackgroundDevnet::spawn_with_additional_args(&["--fork-network", SEPOLIA_URL])
                .await
                .unwrap();

        let resp = &fork_devnet
            .json_rpc_client
            .get_block_with_tx_hashes(BlockId::Hash(
                FieldElement::from_hex_be(SEPOLIA_GENESIS_BLOCK_HASH).unwrap(),
            ))
            .await
            .unwrap();

        match resp {
            MaybePendingBlockWithTxHashes::Block(block) => {
                assert_eq!(block.block_number, 0);
            }
            _ => panic!("Unexpected resp: {resp:?}"),
        };
    }

    #[tokio::test]
    async fn test_getting_non_existent_block_from_origin() {
        let fork_devnet =
            BackgroundDevnet::spawn_with_additional_args(&["--fork-network", SEPOLIA_URL])
                .await
                .expect("Could not start Devnet");

        let non_existent_block_hash = "0x123456";
        let err = &fork_devnet
            .json_rpc_client
            .get_block_with_tx_hashes(BlockId::Hash(
                FieldElement::from_hex_be(non_existent_block_hash).unwrap(),
            ))
            .await
            .expect_err("Should be an error");

        match err {
            ProviderError::StarknetError(StarknetError::BlockNotFound) => (),
            other => panic!("Unexpected error: {other}"),
        }
    }

    #[tokio::test]
    async fn test_forking_local_genesis_block() {
        let origin_devnet =
            BackgroundDevnet::spawn_with_additional_args(&["--state-archive-capacity", "full"])
                .await
                .unwrap();

        let block_creation_resp =
            origin_devnet // TODO replace with a utility method (in other places too)
                .post_json("/create_block".into(), Body::from(json!({}).to_string()))
                .await
                .unwrap();
        let block_creation_resp_body = get_json_body(block_creation_resp).await;
        let block_hash =
            FieldElement::from_hex_be(block_creation_resp_body["block_hash"].as_str().unwrap())
                .unwrap();

        let fork_devnet = origin_devnet.fork().await.unwrap();

        let resp = &fork_devnet
            .json_rpc_client
            .get_block_with_tx_hashes(BlockId::Hash(block_hash))
            .await
            .unwrap();

        match resp {
            MaybePendingBlockWithTxHashes::Block(block) => {
                assert_eq!(block.block_number, 0);
            }
            _ => panic!("Unexpected resp: {resp:?}"),
        };
    }

    #[tokio::test]
    async fn test_forked_account_balance() {
        let origin_devnet =
            BackgroundDevnet::spawn_with_additional_args(&["--state-archive-capacity", "full"])
                .await
                .unwrap();

        // new origin block
        origin_devnet // TODO replace with a utility method
            .post_json("/create_block".into(), Body::from(json!({}).to_string()))
            .await
            .unwrap();

        // new origin block
        let dummy_address = FieldElement::ONE;
        let mint_amount = 100;
        origin_devnet.mint(dummy_address, mint_amount).await;

        let fork_devnet = origin_devnet.fork().await.unwrap();

        // new fork block
        fork_devnet.mint(dummy_address, mint_amount).await;

        for block_i in 0..=1 {
            let block_id = BlockId::Number(block_i);
            let balance = fork_devnet.get_balance_at_block(&dummy_address, block_id).await.unwrap();
            let expected_balance = (block_i as u128 * mint_amount).into();
            assert_eq!(balance, expected_balance);
        }

        // not using get_balance_at_block=2; that requires fork with --state-archive-capacity full
        let final_balance = fork_devnet.get_balance(&dummy_address).await.unwrap();
        let expected_final_balance = (2_u128 * mint_amount).into();
        assert_eq!(final_balance, expected_final_balance);
    }

    async fn get_contract_balance(
        devnet: &BackgroundDevnet,
        contract_address: FieldElement,
    ) -> FieldElement {
        let contract_call = FunctionCall {
            contract_address,
            entry_point_selector: get_selector_from_name("get_balance").unwrap(),
            calldata: vec![],
        };
        match devnet.json_rpc_client.call(contract_call, BlockId::Tag(BlockTag::Latest)).await {
            Ok(res) => {
                assert_eq!(res.len(), 1);
                res[0]
            }
            Err(e) => panic!("Call failed: {e}"),
        }
    }

    #[tokio::test]
    async fn test_getting_cairo0_class_from_origin_and_fork() {
        let origin_devnet =
            BackgroundDevnet::spawn_with_additional_args(&["--state-archive-capacity", "full"])
                .await
                .unwrap();

        let (signer, account_address) = origin_devnet.get_first_predeployed_account().await;
        let predeployed_account = Arc::new(SingleOwnerAccount::new(
            origin_devnet.clone_provider(),
            signer.clone(),
            account_address,
            chain_id::TESTNET,
            ExecutionEncoding::New,
        ));

        let json_string = std::fs::read_to_string(resolve_path(
            "../starknet-devnet-core/test_artifacts/cairo_0_test.json",
        ))
        .unwrap();
        let contract_class: Arc<LegacyContractClass> =
            Arc::new(serde_json::from_str(&json_string).unwrap());

        // declare the contract
        let declaration_result = predeployed_account
            .declare_legacy(contract_class.clone())
            .max_fee(FieldElement::from(1e18 as u128))
            .send()
            .await
            .unwrap();

        let fork_devnet = origin_devnet.fork().await.unwrap();
        let _retrieved_class = fork_devnet
            .json_rpc_client
            .get_class(BlockId::Tag(BlockTag::Latest), declaration_result.class_hash)
            .await
            .unwrap();

        // TODO Currently asserting cairo0 artifacts is failing
        // assert_eq!(retrieved_class, ContractClass::Legacy(contract_class.compress().unwrap()));
    }

    #[tokio::test]
    async fn test_getting_cairo1_class_from_origin_and_fork() {
        let origin_devnet =
            BackgroundDevnet::spawn_with_additional_args(&["--state-archive-capacity", "full"])
                .await
                .unwrap();

        let (signer, account_address) = origin_devnet.get_first_predeployed_account().await;
        let predeployed_account = Arc::new(SingleOwnerAccount::new(
            origin_devnet.clone_provider(),
            signer.clone(),
            account_address,
            chain_id::TESTNET,
            ExecutionEncoding::New,
        ));

        let (contract_class, casm_class_hash) =
            get_simple_contract_in_sierra_and_compiled_class_hash();

        // declare the contract
        let declaration_result = predeployed_account
            .declare(Arc::new(contract_class.clone()), casm_class_hash)
            .max_fee(FieldElement::from(1e18 as u128))
            .send()
            .await
            .unwrap();

        // deploy the contract
        let contract_factory =
            ContractFactory::new(declaration_result.class_hash, predeployed_account.clone());
        let initial_value = FieldElement::from(10_u32);
        let ctor_args = vec![initial_value];
        contract_factory
            .deploy(ctor_args.clone(), FieldElement::ZERO, false)
            .max_fee(FieldElement::from(1e18 as u128))
            .send()
            .await
            .unwrap();

        // generate the address of the newly deployed contract
        let contract_address = get_udc_deployed_address(
            FieldElement::ZERO,
            declaration_result.class_hash,
            &starknet_rs_core::utils::UdcUniqueness::NotUnique,
            &ctor_args,
        );

        let fork_devnet = origin_devnet.fork().await.unwrap();

        let retrieved_class_hash = fork_devnet
            .json_rpc_client
            .get_class_hash_at(BlockId::Tag(BlockTag::Latest), contract_address)
            .await
            .unwrap();
        assert_eq!(retrieved_class_hash, declaration_result.class_hash);

        let retrieved_class = fork_devnet
            .json_rpc_client
            .get_class(BlockId::Tag(BlockTag::Latest), declaration_result.class_hash)
            .await
            .unwrap();
        assert_cairo1_classes_equal(retrieved_class, ContractClass::Sierra(contract_class.clone()))
            .unwrap();
    }

    #[tokio::test]
    async fn test_forking_local_declare_deploy_fork_invoke() {
        let origin_devnet =
            BackgroundDevnet::spawn_with_additional_args(&["--state-archive-capacity", "full"])
                .await
                .unwrap();

        let (signer, account_address) = origin_devnet.get_first_predeployed_account().await;
        let predeployed_account = Arc::new(SingleOwnerAccount::new(
            origin_devnet.clone_provider(),
            signer.clone(),
            account_address,
            chain_id::TESTNET,
            ExecutionEncoding::New,
        ));

        let (contract_class, casm_class_hash) =
            get_simple_contract_in_sierra_and_compiled_class_hash();

        // declare the contract
        let declaration_result = predeployed_account
            .declare(Arc::new(contract_class), casm_class_hash)
            .max_fee(FieldElement::from(1e18 as u128))
            .send()
            .await
            .unwrap();

        // deploy the contract
        let contract_factory =
            ContractFactory::new(declaration_result.class_hash, predeployed_account.clone());
        let initial_value = FieldElement::from(10_u32);
        let ctor_args = vec![initial_value];
        contract_factory
            .deploy(ctor_args.clone(), FieldElement::ZERO, false)
            .max_fee(FieldElement::from(1e18 as u128))
            .send()
            .await
            .unwrap();

        // generate the address of the newly deployed contract
        let contract_address = get_udc_deployed_address(
            FieldElement::ZERO,
            declaration_result.class_hash,
            &starknet_rs_core::utils::UdcUniqueness::NotUnique,
            &ctor_args,
        );

        // assert correctly deployed
        assert_eq!(get_contract_balance(&origin_devnet, contract_address).await, initial_value);

        let fork_devnet = origin_devnet.fork().await.unwrap();

        assert_eq!(get_contract_balance(&fork_devnet, contract_address).await, initial_value);

        let fork_predeployed_account = SingleOwnerAccount::new(
            fork_devnet.clone_provider(),
            signer,
            account_address,
            chain_id::TESTNET,
            ExecutionEncoding::New,
        );

        // invoke on forked devnet
        let increment = FieldElement::from(5_u32);
        let contract_invoke = vec![Call {
            to: contract_address,
            selector: get_selector_from_name("increase_balance").unwrap(),
            calldata: vec![increment, FieldElement::ZERO],
        }];

        let invoke_result = fork_predeployed_account
            .execute(contract_invoke.clone())
            .max_fee(FieldElement::from(1e18 as u128))
            .send()
            .await
            .unwrap();

        assert_tx_successful(&invoke_result.transaction_hash, &fork_devnet.json_rpc_client).await;

        // assert origin intact and fork changed
        assert_eq!(get_contract_balance(&origin_devnet, contract_address).await, initial_value);
        assert_eq!(
            get_contract_balance(&fork_devnet, contract_address).await,
            initial_value + increment
        );
    }

    #[tokio::test]
    async fn test_deploying_account_with_class_not_present_on_origin() {
        let origin_devnet =
            BackgroundDevnet::spawn_with_additional_args(&["--state-archive-capacity", "full"])
                .await
                .unwrap();

        // create forkable origin block
        origin_devnet
            .post_json("/create_block".into(), Body::from(json!({}).to_string()))
            .await
            .unwrap();

        let fork_devnet = origin_devnet.fork().await.unwrap();

        let (signer, _) = origin_devnet.get_first_predeployed_account().await;

        let nonexistent_class_hash = FieldElement::from_hex_be("0x123").unwrap();
        let factory = OpenZeppelinAccountFactory::new(
            nonexistent_class_hash,
            chain_id::TESTNET,
            signer,
            fork_devnet.clone_provider(),
        )
        .await
        .unwrap();

        let salt = FieldElement::from_hex_be("0x123").unwrap();
        let deployment =
            factory.deploy(salt).max_fee(FieldElement::from(1e18 as u128)).send().await;
        match deployment {
            Err(AccountFactoryError::Provider(ProviderError::StarknetError(
                StarknetError::ClassHashNotFound,
            ))) => (),
            unexpected => panic!("Unexpected resp: {unexpected:?}"),
        }
    }

    #[tokio::test]
    async fn test_deploying_account_with_class_present_on_origin() {
        let origin_devnet = BackgroundDevnet::spawn_with_additional_args(&[
            "--state-archive-capacity",
            "full",
            "--account-class",
            "cairo1",
        ])
        .await
        .unwrap();

        // create forkable origin block
        origin_devnet
            .post_json("/create_block".into(), Body::from(json!({}).to_string()))
            .await
            .unwrap();

        let (signer, _) = origin_devnet.get_first_predeployed_account().await;

        let fork_devnet = origin_devnet.fork().await.unwrap();

        // deploy account using class from origin
        let factory = OpenZeppelinAccountFactory::new(
            FieldElement::from_hex_be(CAIRO_0_ACCOUNT_CONTRACT_HASH).unwrap(),
            chain_id::TESTNET,
            signer,
            fork_devnet.clone_provider(),
        )
        .await
        .unwrap();

        let salt = FieldElement::from_hex_be("0x123").unwrap();
        let deployment = factory.deploy(salt).max_fee(FieldElement::from(1e18 as u128));
        let deployment_address = deployment.address();
        fork_devnet.mint(deployment_address, 1e18 as u128).await;
        deployment.send().await.unwrap();
    }
}
