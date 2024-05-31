// must use `pub`: https://github.com/rust-lang/rust/issues/46379#issuecomment-548787629
pub mod common;

mod test_account_selection {
    use std::sync::Arc;

    use starknet_core::constants::{
        CAIRO_1_ACCOUNT_CONTRACT_SIERRA_HASH, CAIRO_1_ACCOUNT_CONTRACT_SIERRA_PATH,
    };
    use starknet_core::utils::exported_test_utils::dummy_cairo_0_contract_class;
    use starknet_rs_accounts::{
        Account, AccountFactory, ArgentAccountFactory, Call, ExecutionEncoding,
        OpenZeppelinAccountFactory, SingleOwnerAccount,
    };
    use starknet_rs_contract::ContractFactory;
    use starknet_rs_core::types::contract::legacy::LegacyContractClass;
    use starknet_rs_core::types::{
        BlockId, BlockTag, DeployAccountTransactionResult, FieldElement, FunctionCall,
    };
    use starknet_rs_core::utils::{
        get_selector_from_name, get_udc_deployed_address, UdcUniqueness,
    };
    use starknet_rs_providers::Provider;
    use starknet_rs_signers::LocalWallet;

    use crate::common::background_devnet::BackgroundDevnet;
    use crate::common::constants::{CHAIN_ID, SEPOLIA_URL};
    use crate::common::utils::{
        assert_tx_successful, get_deployable_account_signer,
        get_simple_contract_in_sierra_and_compiled_class_hash,
    };

    enum AccountClassSelection {
        OpenZeppelin,
        Argent,
    }

    impl AccountClassSelection {
        fn get_class_hash(&self) -> FieldElement {
            FieldElement::from_hex_be(match self {
                AccountClassSelection::OpenZeppelin => CAIRO_1_ACCOUNT_CONTRACT_SIERRA_HASH,
                AccountClassSelection::Argent => {
                    "0x01a736d6ed154502257f02b1ccdf4d9d1089f80811cd6acad48e6b6a9d1f2003"
                }
            })
            .unwrap()
        }

        async fn deploy(
            &self,
            devnet: &BackgroundDevnet,
        ) -> (DeployAccountTransactionResult, LocalWallet) {
            let signer = get_deployable_account_signer();

            let salt = FieldElement::THREE;
            let deployment = match self {
                AccountClassSelection::OpenZeppelin => {
                    let factory = OpenZeppelinAccountFactory::new(
                        self.get_class_hash(),
                        CHAIN_ID,
                        signer.clone(),
                        devnet.clone_provider(),
                    )
                    .await
                    .unwrap();

                    let deployment = factory.deploy(salt);

                    let account_address = deployment.address();
                    devnet.mint(account_address, 1e18 as u128).await;
                    deployment.send().await.unwrap()
                }
                AccountClassSelection::Argent => {
                    let factory = ArgentAccountFactory::new(
                        self.get_class_hash(),
                        CHAIN_ID,
                        FieldElement::ZERO,
                        signer.clone(),
                        devnet.clone_provider(),
                    )
                    .await
                    .unwrap();

                    let deployment = factory.deploy(salt);

                    let account_address = deployment.address();
                    devnet.mint(account_address, 1e18 as u128).await;
                    deployment.send().await.unwrap()
                }
            };

            (deployment, signer)
        }
    }

    #[tokio::test]
    async fn spawnable_with_cairo0() {
        BackgroundDevnet::spawn_with_additional_args(&["--account-class", "cairo0"]).await.unwrap();
    }

    #[tokio::test]
    async fn spawnable_with_cairo1() {
        BackgroundDevnet::spawn_with_additional_args(&["--account-class", "cairo1"]).await.unwrap();
    }

    #[tokio::test]
    async fn spawnable_with_custom_account_cairo_1() {
        let args = ["--account-class-custom", CAIRO_1_ACCOUNT_CONTRACT_SIERRA_PATH];
        BackgroundDevnet::spawn_with_additional_args(&args).await.unwrap();
    }

    /// Common body for tests defined below
    async fn correct_artifact_test_body(devnet_args: &[&str], expected_hash_hex: &str) {
        let devnet = BackgroundDevnet::spawn_with_additional_args(devnet_args).await.unwrap();

        let (_, account_address) = devnet.get_first_predeployed_account().await;
        let retrieved_class_hash = devnet
            .json_rpc_client
            .get_class_hash_at(BlockId::Tag(BlockTag::Latest), account_address)
            .await
            .unwrap();
        let expected_hash = FieldElement::from_hex_be(expected_hash_hex).unwrap();
        assert_eq!(retrieved_class_hash, expected_hash);

        let config = devnet.get_config().await.unwrap();
        let config_class_hash_hex = config["account_contract_class_hash"].as_str().unwrap();
        assert_eq!(FieldElement::from_hex_be(config_class_hash_hex).unwrap(), expected_hash);
    }

    #[tokio::test]
    async fn correct_cairo1_artifact() {
        let args = ["--account-class", "cairo1"];
        correct_artifact_test_body(&args, CAIRO_1_ACCOUNT_CONTRACT_SIERRA_HASH).await;
    }

    #[tokio::test]
    async fn correct_custom_artifact() {
        let args = ["--account-class-custom", CAIRO_1_ACCOUNT_CONTRACT_SIERRA_PATH];
        correct_artifact_test_body(&args, CAIRO_1_ACCOUNT_CONTRACT_SIERRA_HASH).await;
    }

    /// Common body for tests defined below
    async fn can_deploy_new_account_test_body(
        devnet_args: &[&str],
        account_class_selection: AccountClassSelection,
    ) {
        let devnet = BackgroundDevnet::spawn_with_additional_args(devnet_args).await.unwrap();

        let (account_deployment, signer) = account_class_selection.deploy(&devnet).await;

        let deploy_account_receipt = devnet
            .json_rpc_client
            .get_transaction_receipt(account_deployment.transaction_hash)
            .await
            .unwrap();

        assert_tx_successful(deploy_account_receipt.transaction_hash(), &devnet.json_rpc_client)
            .await;

        can_declare_deploy_invoke_cairo0_using_account(
            &devnet,
            &signer,
            account_deployment.contract_address,
        )
        .await;

        can_declare_deploy_invoke_cairo1_using_account(
            &devnet,
            &signer,
            account_deployment.contract_address,
        )
        .await;
    }

    #[tokio::test]
    async fn can_deploy_new_cairo1_oz_account() {
        can_deploy_new_account_test_body(
            &["--account-class", "cairo1"],
            AccountClassSelection::OpenZeppelin,
        )
        .await;
    }

    #[tokio::test]
    async fn can_deploy_new_cairo1_oz_account_when_cairo0_selected() {
        can_deploy_new_account_test_body(
            &["--account-class", "cairo0"],
            AccountClassSelection::OpenZeppelin,
        )
        .await;
    }

    #[tokio::test]
    async fn can_deploy_new_custom_oz_account() {
        let args = ["--account-class-custom", CAIRO_1_ACCOUNT_CONTRACT_SIERRA_PATH];
        can_deploy_new_account_test_body(&args, AccountClassSelection::OpenZeppelin).await;
    }

    #[tokio::test]
    async fn can_deploy_new_argent_account() {
        let args = ["--fork-network", SEPOLIA_URL];
        can_deploy_new_account_test_body(&args, AccountClassSelection::Argent).await;
    }

    async fn can_declare_deploy_invoke_cairo0_using_account(
        devnet: &BackgroundDevnet,
        signer: &LocalWallet,
        account_address: FieldElement,
    ) {
        let account = Arc::new(SingleOwnerAccount::new(
            devnet.clone_provider(),
            signer,
            account_address,
            CHAIN_ID,
            ExecutionEncoding::New,
        ));

        // get class
        let contract_json = dummy_cairo_0_contract_class();
        let contract_artifact: Arc<LegacyContractClass> =
            Arc::new(serde_json::from_value(contract_json.inner).unwrap());
        let class_hash = contract_artifact.class_hash().unwrap();

        // declare class
        let declaration_result =
            account.declare_legacy(contract_artifact.clone()).send().await.unwrap();
        assert_eq!(declaration_result.class_hash, class_hash);

        // deploy instance of class
        let contract_factory = ContractFactory::new(class_hash, account.clone());
        let salt = FieldElement::from_hex_be("0x123").unwrap();
        let constructor_calldata = vec![];
        let contract_address = get_udc_deployed_address(
            salt,
            class_hash,
            &UdcUniqueness::NotUnique,
            &constructor_calldata,
        );
        contract_factory
            .deploy(constructor_calldata, salt, false)
            .send()
            .await
            .expect("Cannot deploy");

        // invoke
        let increase_amount = FieldElement::from(100u128);
        let invoke_calls = vec![Call {
            to: contract_address,
            selector: get_selector_from_name("increase_balance").unwrap(),
            calldata: vec![increase_amount],
        }];
        account.execute(invoke_calls).send().await.unwrap();

        // prepare the call used in checking the balance
        let call = FunctionCall {
            contract_address,
            entry_point_selector: get_selector_from_name("get_balance").unwrap(),
            calldata: vec![],
        };
        let balance_after_sufficient =
            devnet.json_rpc_client.call(call, BlockId::Tag(BlockTag::Latest)).await.unwrap();
        assert_eq!(balance_after_sufficient, vec![increase_amount]);
    }

    async fn can_declare_deploy_invoke_cairo1_using_account(
        devnet: &BackgroundDevnet,
        signer: &LocalWallet,
        account_address: FieldElement,
    ) {
        let account = Arc::new(SingleOwnerAccount::new(
            devnet.clone_provider(),
            signer,
            account_address,
            CHAIN_ID,
            ExecutionEncoding::New,
        ));

        let (contract_class, casm_hash) = get_simple_contract_in_sierra_and_compiled_class_hash();

        // declare the contract
        let declaration_result =
            account.declare(Arc::new(contract_class), casm_hash).send().await.unwrap();

        // deploy the contract
        let contract_factory = ContractFactory::new(declaration_result.class_hash, account.clone());
        let initial_value = FieldElement::from(10_u32);
        let ctor_args = vec![initial_value];
        contract_factory.deploy(ctor_args.clone(), FieldElement::ZERO, false).send().await.unwrap();

        // generate the address of the newly deployed contract
        let contract_address = get_udc_deployed_address(
            FieldElement::ZERO,
            declaration_result.class_hash,
            &starknet_rs_core::utils::UdcUniqueness::NotUnique,
            &ctor_args,
        );

        // invoke on forked devnet
        let increment = FieldElement::from(5_u32);
        let contract_invoke = vec![Call {
            to: contract_address,
            selector: get_selector_from_name("increase_balance").unwrap(),
            calldata: vec![increment, FieldElement::ZERO],
        }];

        let invoke_result = account.execute(contract_invoke.clone()).send().await.unwrap();

        assert_tx_successful(&invoke_result.transaction_hash, &devnet.json_rpc_client).await;
    }

    /// Common body for tests defined below
    async fn can_declare_deploy_invoke_using_predeployed_test_body(devnet_args: &[&str]) {
        let devnet = BackgroundDevnet::spawn_with_additional_args(devnet_args).await.unwrap();

        // get account
        let (signer, account_address) = devnet.get_first_predeployed_account().await;
        can_declare_deploy_invoke_cairo0_using_account(&devnet, &signer, account_address).await;
        can_declare_deploy_invoke_cairo1_using_account(&devnet, &signer, account_address).await;
    }

    #[tokio::test]
    async fn can_declare_deploy_invoke_using_predeployed_cairo1() {
        can_declare_deploy_invoke_using_predeployed_test_body(&["--account-class", "cairo1"]).await;
    }

    #[tokio::test]
    async fn can_declare_deploy_invoke_using_predeployed_custom() {
        let args = ["--account-class-custom", CAIRO_1_ACCOUNT_CONTRACT_SIERRA_PATH];
        can_declare_deploy_invoke_using_predeployed_test_body(&args).await;
    }

    async fn assert_supports_isrc6(devnet: &BackgroundDevnet, account_address: FieldElement) {
        // https://github.com/OpenZeppelin/cairo-contracts/blob/89a450a88628ec3b86273f261b2d8d1ca9b1522b/src/account/interface.cairo#L7
        let interface_id_hex = "0x2ceccef7f994940b3962a6c67e0ba4fcd37df7d131417c604f91e03caecc1cd";
        let interface_id = FieldElement::from_hex_be(interface_id_hex).unwrap();

        let call = FunctionCall {
            contract_address: account_address,
            entry_point_selector: get_selector_from_name("supports_interface").unwrap(),
            calldata: vec![interface_id],
        };

        let supports =
            devnet.json_rpc_client.call(call, BlockId::Tag(BlockTag::Latest)).await.unwrap();
        assert_eq!(supports, vec![FieldElement::ONE]);
    }

    #[tokio::test]
    async fn test_interface_support_of_predeployed_account() {
        let devnet = BackgroundDevnet::spawn().await.unwrap();
        let (_, account_address) = devnet.get_first_predeployed_account().await;

        assert_supports_isrc6(&devnet, account_address).await;
    }

    #[tokio::test]
    async fn test_interface_support_of_newly_deployed_account() {
        let devnet = BackgroundDevnet::spawn().await.unwrap();

        let (account_deployment, _) = AccountClassSelection::OpenZeppelin.deploy(&devnet).await;

        assert_supports_isrc6(&devnet, account_deployment.contract_address).await;
    }
}
