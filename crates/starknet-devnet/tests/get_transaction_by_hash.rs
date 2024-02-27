pub mod common;

mod get_transaction_by_hash_integration_tests {
    use std::sync::Arc;

    use starknet_core::constants::{CAIRO_0_ACCOUNT_CONTRACT_HASH, ETH_ERC20_CONTRACT_ADDRESS};
    use starknet_rs_accounts::{
        Account, AccountFactory, Call, ExecutionEncoding, OpenZeppelinAccountFactory,
        SingleOwnerAccount,
    };
    use starknet_rs_core::chain_id;
    use starknet_rs_core::types::contract::legacy::LegacyContractClass;
    use starknet_rs_core::types::contract::{CompiledClass, SierraClass};
    use starknet_rs_core::types::{BlockId, BlockTag, FieldElement, StarknetError};
    use starknet_rs_core::utils::get_selector_from_name;
    use starknet_rs_providers::{Provider, ProviderError};
    use starknet_types::contract_class::Cairo0Json;
    use starknet_types::felt::Felt;
    use starknet_types::traits::ToHexString;

    use crate::common::background_devnet::BackgroundDevnet;
    use crate::common::constants::CASM_COMPILED_CLASS_HASH;
    use crate::common::utils::{get_deployable_account_signer, resolve_path};

    #[tokio::test]
    async fn get_declare_v1_transaction_by_hash_happy_path() {
        let devnet = BackgroundDevnet::spawn().await.expect("Could not start Devnet");
        let json_string = std::fs::read_to_string(resolve_path(
            "../starknet-devnet-core/test_artifacts/cairo_0_test.json",
        ))
        .unwrap();

        let legacy_contract_class = Cairo0Json::raw_json_from_json_str(&json_string).unwrap();
        let legacy_contract_class: LegacyContractClass =
            serde_json::from_value(legacy_contract_class.inner).unwrap();

        let (signer, account_address) = devnet.get_first_predeployed_account().await;

        let mut account = SingleOwnerAccount::new(
            &devnet.json_rpc_client,
            signer,
            account_address,
            chain_id::TESTNET,
            ExecutionEncoding::Legacy,
        );
        account.set_block_id(BlockId::Tag(BlockTag::Latest));

        let declare_transaction = account
            .declare_legacy(Arc::new(legacy_contract_class))
            .nonce(FieldElement::ZERO)
            .send()
            .await
            .unwrap();

        let result = devnet
            .json_rpc_client
            .get_transaction_by_hash(declare_transaction.transaction_hash)
            .await
            .unwrap();

        if let starknet_rs_core::types::Transaction::Declare(
            starknet_rs_core::types::DeclareTransaction::V1(declare_v1),
        ) = result
        {
            let expected = "0x07afcbaed2ffa59b36ded08df59c7e55c2b237cd2bc1d12869d742cf6ad923c2";
            assert_eq!(declare_v1.transaction_hash, FieldElement::from_hex_be(expected).unwrap());
        } else {
            panic!("Could not unpack the transaction from {result:?}");
        }
    }

    #[tokio::test]
    async fn get_declare_v2_transaction_by_hash_happy_path() {
        let devnet = BackgroundDevnet::spawn().await.expect("Could not start Devnet");

        // Sierra class artifact. Output of the `starknet-compile` command.
        let path_to_cairo1 =
            concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/rpc/contract_cairo_v1/output.json");
        let contract_artifact: SierraClass =
            serde_json::from_reader(std::fs::File::open(path_to_cairo1).unwrap()).unwrap();

        // Casm artifact. Output of the `starknet-sierra-compile` command.
        let path_to_casm = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/test_data/rpc/contract_cairo_v1/output-casm.json"
        );
        let casm_contract_definition: CompiledClass =
            serde_json::from_reader(std::fs::File::open(path_to_casm).unwrap()).unwrap();
        let compiled_class_hash = (casm_contract_definition.class_hash()).unwrap();
        assert_eq!(Felt::from(compiled_class_hash).to_prefixed_hex_str(), CASM_COMPILED_CLASS_HASH);

        let (signer, address) = devnet.get_first_predeployed_account().await;
        let mut account = SingleOwnerAccount::new(
            &devnet.json_rpc_client,
            signer,
            address,
            chain_id::TESTNET,
            ExecutionEncoding::Legacy,
        );
        account.set_block_id(BlockId::Tag(BlockTag::Latest));

        // We need to flatten the ABI into a string first
        let flattened_class = contract_artifact.flatten().unwrap();
        let declare_result = account
            .declare(Arc::new(flattened_class), compiled_class_hash)
            .nonce(FieldElement::ZERO)
            .send()
            .await;

        let result = devnet
            .json_rpc_client
            .get_transaction_by_hash(declare_result.unwrap().transaction_hash)
            .await
            .unwrap();

        if let starknet_rs_core::types::Transaction::Declare(
            starknet_rs_core::types::DeclareTransaction::V2(declare_v2),
        ) = result
        {
            let expected = "0x0376392f405221ebbe3d9c50366b8fe62f403a9613f0a1cddd6210ffb6e46632";
            assert_eq!(declare_v2.transaction_hash, FieldElement::from_hex_be(expected).unwrap());
        } else {
            panic!("Could not unpack the transaction from {result:?}");
        }
    }

    #[tokio::test]
    async fn get_deploy_account_transaction_by_hash_happy_path() {
        let devnet = BackgroundDevnet::spawn().await.expect("Could not start Devnet");

        let signer = get_deployable_account_signer();

        let factory = OpenZeppelinAccountFactory::new(
            FieldElement::from_hex_be(CAIRO_0_ACCOUNT_CONTRACT_HASH).unwrap(),
            chain_id::TESTNET,
            signer,
            devnet.clone_provider(),
        )
        .await
        .unwrap();

        let salt = FieldElement::from_hex_be("0x123").unwrap();
        let deployment = factory.deploy(salt);
        let deployment_address = deployment.address();
        let fee_estimation =
            factory.deploy(salt).fee_estimate_multiplier(1.0).estimate_fee().await.unwrap();

        // fund the account before deployment
        let mint_amount = fee_estimation.overall_fee * FieldElement::TWO;
        devnet.mint(deployment_address, mint_amount.try_into().unwrap()).await;

        let deploy_account_result = deployment.send().await.unwrap();

        let result = devnet
            .json_rpc_client
            .get_transaction_by_hash(deploy_account_result.transaction_hash)
            .await
            .unwrap();

        if let starknet_rs_core::types::Transaction::DeployAccount(deploy) = result {
            let expected = "0x011ce9d9fab9d0fccd846ce0a7698da19ace2b91cb3db2df1c8845904f74af91";
            assert_eq!(*deploy.transaction_hash(), FieldElement::from_hex_be(expected).unwrap());
        } else {
            panic!("Could not unpack the transaction from {result:?}");
        }
    }

    #[tokio::test]
    async fn get_invoke_v1_transaction_by_hash_happy_path() {
        let devnet = BackgroundDevnet::spawn().await.expect("Could not start Devnet");
        let (signer, account_address) = devnet.get_first_predeployed_account().await;

        let account = SingleOwnerAccount::new(
            &devnet.json_rpc_client,
            signer,
            account_address,
            chain_id::TESTNET,
            ExecutionEncoding::Legacy,
        );

        let invoke_transaction = account
            .execute(vec![Call {
                to: FieldElement::from_hex_be(ETH_ERC20_CONTRACT_ADDRESS).unwrap(),
                selector: get_selector_from_name("transfer").unwrap(),
                calldata: vec![
                    FieldElement::ONE,                                 // recipient
                    FieldElement::from_dec_str("1000000000").unwrap(), // low part of uint256
                    FieldElement::ZERO,                                // high part of uint256
                ],
            }])
            .send()
            .await
            .unwrap();

        let result = devnet
            .json_rpc_client
            .get_transaction_by_hash(invoke_transaction.transaction_hash)
            .await
            .unwrap();

        if let starknet_rs_core::types::Transaction::Invoke(
            starknet_rs_core::types::InvokeTransaction::V1(invoke_v1),
        ) = result
        {
            let expected = "0x07e5bbb4f29a83fded43e3f9fc38a3864d98a2817fd2c1b1164d118a88a65158";
            assert_eq!(invoke_v1.transaction_hash, FieldElement::from_hex_be(expected).unwrap());
        } else {
            panic!("Could not unpack the transaction from {result:?}");
        }
    }

    #[tokio::test]
    async fn get_non_existing_transaction() {
        let devnet = BackgroundDevnet::spawn().await.expect("Could not start Devnet");
        let result = devnet
            .json_rpc_client
            .get_transaction_by_hash(FieldElement::from_hex_be("0x0").unwrap())
            .await
            .unwrap_err();

        match result {
            ProviderError::StarknetError(StarknetError::TransactionHashNotFound) => (),
            _ => panic!("Invalid error: {result:?}"),
        }
    }
}
