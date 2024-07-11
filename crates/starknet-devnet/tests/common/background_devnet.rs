use std::collections::HashMap;
use std::fmt::LowerHex;
use std::net::TcpListener;
use std::process::{Child, Command, Stdio};
use std::time;

use lazy_static::lazy_static;
use reqwest::{Client, StatusCode};
use serde_json::json;
use server::rpc_core::error::RpcError;
use starknet_core::constants::ETH_ERC20_CONTRACT_ADDRESS;
use starknet_rs_core::types::{
    BlockId, BlockTag, BlockWithTxHashes, BlockWithTxs, FieldElement, FunctionCall,
    MaybePendingBlockWithTxHashes, MaybePendingBlockWithTxs, PendingBlockWithTxHashes,
    PendingBlockWithTxs,
};
use starknet_rs_core::utils::get_selector_from_name;
use starknet_rs_providers::jsonrpc::HttpTransport;
use starknet_rs_providers::{JsonRpcClient, Provider};
use starknet_rs_signers::{LocalWallet, SigningKey};
use starknet_types::felt::Felt;
use starknet_types::num_bigint::BigUint;
use starknet_types::rpc::transaction_receipt::FeeUnit;
use tokio::sync::Mutex;
use url::Url;

use super::constants::{
    ACCOUNTS, CHAIN_ID_CLI_PARAM, HEALTHCHECK_PATH, HOST, MAX_PORT, MIN_PORT,
    PREDEPLOYED_ACCOUNT_INITIAL_BALANCE, RPC_PATH, SEED,
};
use super::errors::TestError;
use super::reqwest_client::{PostReqwestSender, ReqwestClient};
use super::utils::{to_hex_felt, ImpersonationAction};

lazy_static! {
    /// This is to prevent TOCTOU errors; i.e. one background devnet might find one
    /// port to be free, and while it's trying to start listening to it, another instance
    /// finds that it's free and tries occupying it
    /// Using the mutex in `get_free_port_listener` might be safer than using no mutex at all,
    /// but not sufficiently safe
    static ref BACKGROUND_DEVNET_MUTEX: Mutex<()> = Mutex::new(());
}

#[derive(Debug)]
pub struct BackgroundDevnet {
    pub reqwest_client: ReqwestClient,
    pub json_rpc_client: JsonRpcClient<HttpTransport>,
    pub process: Child,
    pub port: u16,
    pub url: String,
    rpc_url: Url,
}

fn get_free_port() -> Result<u16, TestError> {
    for port in MIN_PORT..=MAX_PORT {
        if let Ok(listener) = TcpListener::bind(("127.0.0.1", port)) {
            return Ok(listener.local_addr().expect("No local addr").port());
        }
        // otherwise port is occupied
    }
    Err(TestError::NoFreePorts)
}

lazy_static! {
    static ref DEFAULT_CLI_MAP: HashMap<&'static str, String> = HashMap::from([
        ("--seed", SEED.to_string()),
        ("--accounts", ACCOUNTS.to_string()),
        ("--initial-balance", PREDEPLOYED_ACCOUNT_INITIAL_BALANCE.to_string()),
        ("--chain-id", CHAIN_ID_CLI_PARAM.to_string())
    ]);

    // key is the element that must not be part of the CLI arguments if the value is present, when constructing the CLI arguments for starting background devnet
    static ref CONFLICTING_CLI_SETTINGS: HashMap<&'static str, &'static str> = HashMap::from([
        ("--chain-id", "--fork-network")
    ]);
}

impl BackgroundDevnet {
    /// Ensures the background instance spawns at a free port, checks at most `MAX_RETRIES`
    /// times
    #[allow(dead_code)] // dead_code needed to pass clippy
    pub(crate) async fn spawn() -> Result<Self, TestError> {
        BackgroundDevnet::spawn_with_additional_args(&[]).await
    }

    pub async fn spawn_forkable_devnet() -> Result<BackgroundDevnet, anyhow::Error> {
        let args = ["--state-archive-capacity", "full"];
        let devnet = BackgroundDevnet::spawn_with_additional_args(&args).await?;
        Ok(devnet)
    }

    pub fn reqwest_client(&self) -> &ReqwestClient {
        &self.reqwest_client
    }

    /// Takes specified args and adds default values for args that are missing
    fn add_default_args<'a>(specified_args: &[&'a str]) -> Vec<&'a str> {
        let specified_args_map: HashMap<&str, &str> =
            specified_args.to_vec().chunks_exact(2).map(|chunk| (chunk[0], chunk[1])).collect();

        // filter out default cli settings that are either:
        // - in the specified args
        // - in the specified args have a conflicting CLI param with the default settings
        let modified_default_args_map: HashMap<&str, &str> = DEFAULT_CLI_MAP
            .iter()
            .filter(|(arg_name, _)| {
                let element_not_present_in_specified_args =
                    !specified_args_map.contains_key(*arg_name);
                if !element_not_present_in_specified_args {
                    return false;
                }

                let a = if let Some(conflicting_arg) = CONFLICTING_CLI_SETTINGS.get(*arg_name) {
                    specified_args_map.contains_key(conflicting_arg)
                } else {
                    false
                };
                if !a {
                    return false;
                }

                true
            })
            .map(|(arg_name, default_value)| (*arg_name, default_value.as_str()))
            .collect();

        let mut final_args: Vec<&str> = vec![];
        for (arg_name, arg_value) in
            specified_args_map.iter().chain(modified_default_args_map.iter())
        {
            final_args.push(arg_name);
            final_args.push(arg_value);
        }

        final_args
    }

    pub(crate) async fn spawn_with_additional_args(args: &[&str]) -> Result<Self, TestError> {
        // we keep the reference, otherwise the mutex unlocks immediately
        let _mutex_guard = BACKGROUND_DEVNET_MUTEX.lock().await;

        let free_port = get_free_port().expect("No free ports");

        let devnet_url = format!("http://{HOST}:{free_port}");
        let devnet_rpc_url = Url::parse(format!("{}{RPC_PATH}", devnet_url.as_str()).as_str())?;
        let json_rpc_client = JsonRpcClient::new(HttpTransport::new(devnet_rpc_url.clone()));

        let process = Command::new("cargo")
                .arg("run")
                .arg("--release")
                .arg("--")
                .arg("--port")
                .arg(free_port.to_string())
                .args(Self::add_default_args(args))
                .stdout(Stdio::piped()) // comment this out for complete devnet stdout
                .spawn()
                .expect("Could not start background devnet");

        let healthcheck_uri = format!("{}{HEALTHCHECK_PATH}", devnet_url.as_str()).to_string();
        let reqwest_client = Client::new();

        let max_retries = 30;
        for _ in 0..max_retries {
            if let Ok(alive_resp) = reqwest_client.get(&healthcheck_uri).send().await {
                assert_eq!(alive_resp.status(), StatusCode::OK);
                println!("Spawned background devnet at port {free_port}");
                return Ok(BackgroundDevnet {
                    reqwest_client: ReqwestClient::new(devnet_url.clone(), reqwest_client),
                    json_rpc_client,
                    process,
                    port: free_port,
                    url: devnet_url,
                    rpc_url: devnet_rpc_url,
                });
            }

            // If still in the loop, there is an error: probably a ConnectError if Devnet is not yet
            // up so we retry after some sleep.
            tokio::time::sleep(time::Duration::from_millis(500)).await;
        }

        Err(TestError::DevnetNotStartable)
    }

    pub async fn send_custom_rpc(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, RpcError> {
        let body_json = if params.is_null() {
            json!({
                "jsonrpc": "2.0",
                "id": 0,
                "method": method
            })
        } else {
            json!({
                "jsonrpc": "2.0",
                "id": 0,
                "method": method,
                "params": params
            })
        };

        let json_rpc_result: serde_json::Value = self
            .reqwest_client()
            .post_json_async(RPC_PATH, body_json)
            .await
            .map_err(|err| RpcError::internal_error_with(err.error_message()))?;

        if let Some(result) = json_rpc_result.get("result") {
            Ok(result.clone())
        } else if let Some(error) = json_rpc_result.get("error") {
            Err(serde_json::from_value(error.clone()).unwrap())
        } else {
            Err(RpcError::internal_error_with("Server responded with malformed response"))
        }
    }

    pub fn clone_provider(&self) -> JsonRpcClient<HttpTransport> {
        JsonRpcClient::new(HttpTransport::new(self.rpc_url.clone()))
    }

    pub async fn mint(&self, address: impl LowerHex, mint_amount: u128) -> FieldElement {
        self.mint_unit(address, mint_amount, FeeUnit::WEI).await
    }

    pub async fn mint_unit(
        &self,
        address: impl LowerHex,
        mint_amount: u128,
        unit: FeeUnit,
    ) -> FieldElement {
        let resp_body: serde_json::Value = self
            .send_custom_rpc(
                "devnet_mint",
                json!({
                    "address": format!("{address:#x}"),
                    "amount": mint_amount,
                    "unit": unit,
                }),
            )
            .await
            .unwrap();

        FieldElement::from_hex_be(resp_body["tx_hash"].as_str().unwrap()).unwrap()
    }

    /// Get ETH balance at contract_address, as written in ERC20
    pub async fn get_balance_at_block(
        &self,
        address: &FieldElement,
        block_id: BlockId,
    ) -> Result<FieldElement, anyhow::Error> {
        let call = FunctionCall {
            contract_address: FieldElement::from_hex_be(ETH_ERC20_CONTRACT_ADDRESS).unwrap(),
            entry_point_selector: get_selector_from_name("balanceOf").unwrap(),
            calldata: vec![*address],
        };
        let balance_raw = self.json_rpc_client.call(call, block_id).await?;
        assert_eq!(balance_raw.len(), 2);
        let balance_low: BigUint = (Felt::from(*balance_raw.get(0).unwrap())).into();
        let balance_high: BigUint = (Felt::from(*balance_raw.get(1).unwrap())).into();
        let balance: BigUint = (balance_high << 128) + balance_low;
        Ok(FieldElement::from_byte_slice_be(&balance.to_bytes_be())?)
    }

    /// Get balance at contract_address, as written in the ERC20 contract corresponding to `unit`
    /// from latest state
    pub async fn get_balance_latest(
        &self,
        address: &FieldElement,
        unit: FeeUnit,
    ) -> Result<FieldElement, anyhow::Error> {
        Self::get_balance_by_tag(self, address, unit, BlockTag::Latest).await
    }

    /// Get balance at contract_address, as written in the ERC20 contract corresponding to `unit`
    /// from pending state or latest state
    pub async fn get_balance_by_tag(
        &self,
        address: &FieldElement,
        unit: FeeUnit,
        tag: BlockTag,
    ) -> Result<FieldElement, anyhow::Error> {
        let json_resp = self
            .send_custom_rpc(
                "devnet_getAccountBalance",
                json!({
                    "address": format!("{address:#x}"),
                    "unit": unit,
                    "block_tag": Self::tag_to_str(tag)
                }),
            )
            .await
            .unwrap();

        // response validity asserted in test_balance.rs::assert_balance_endpoint_response
        let amount_raw = json_resp["amount"].as_str().unwrap();
        Ok(FieldElement::from_dec_str(amount_raw)?)
    }

    fn tag_to_str(tag: BlockTag) -> &'static str {
        match tag {
            BlockTag::Latest => "latest",
            BlockTag::Pending => "pending",
        }
    }

    /// This method returns the private key and the address of the first predeployed account
    pub async fn get_first_predeployed_account(&self) -> (LocalWallet, FieldElement) {
        let predeployed_accounts_json =
            self.send_custom_rpc("devnet_getPredeployedAccounts", json!({})).await.unwrap();

        let first_account = predeployed_accounts_json.as_array().unwrap().get(0).unwrap();

        let account_address =
            FieldElement::from_hex_be(first_account["address"].as_str().unwrap()).unwrap();
        let private_key =
            FieldElement::from_hex_be(first_account["private_key"].as_str().unwrap()).unwrap();

        let signer = LocalWallet::from(SigningKey::from_secret_scalar(private_key));

        (signer, account_address)
    }

    pub async fn restart(&self) {
        self.send_custom_rpc("devnet_restart", json!({})).await.unwrap();
    }

    pub async fn fork(&self) -> Result<Self, TestError> {
        let args = ["--fork-network", self.url.as_str(), "--accounts", "0"];
        BackgroundDevnet::spawn_with_additional_args(&args).await
    }

    pub async fn fork_with_full_state_archive(&self) -> Result<Self, TestError> {
        let args = [
            "--fork-network",
            self.url.as_str(),
            "--accounts",
            "0",
            "--state-archive-capacity",
            "full",
        ];
        BackgroundDevnet::spawn_with_additional_args(&args).await
    }

    /// Mines a new block and returns its hash
    pub async fn create_block(&self) -> Result<FieldElement, anyhow::Error> {
        let block_creation_resp_body: serde_json::Value =
            self.send_custom_rpc("devnet_createBlock", json!({})).await.unwrap();

        let block_hash_str = block_creation_resp_body["block_hash"].as_str().unwrap();
        Ok(FieldElement::from_hex_be(block_hash_str)?)
    }

    pub async fn get_latest_block_with_tx_hashes(
        &self,
    ) -> Result<BlockWithTxHashes, anyhow::Error> {
        match self.json_rpc_client.get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest)).await {
            Ok(MaybePendingBlockWithTxHashes::Block(b)) => Ok(b),
            other => Err(anyhow::format_err!("Got unexpected block: {other:?}")),
        }
    }

    pub async fn get_pending_block_with_tx_hashes(
        &self,
    ) -> Result<PendingBlockWithTxHashes, anyhow::Error> {
        match self.json_rpc_client.get_block_with_tx_hashes(BlockId::Tag(BlockTag::Pending)).await {
            Ok(MaybePendingBlockWithTxHashes::PendingBlock(b)) => Ok(b),
            other => Err(anyhow::format_err!("Got unexpected block: {other:?}")),
        }
    }

    pub async fn get_latest_block_with_txs(&self) -> Result<BlockWithTxs, anyhow::Error> {
        match self.json_rpc_client.get_block_with_txs(BlockId::Tag(BlockTag::Latest)).await {
            Ok(MaybePendingBlockWithTxs::Block(b)) => Ok(b),
            other => Err(anyhow::format_err!("Got unexpected block: {other:?}")),
        }
    }

    pub async fn get_pending_block_with_txs(&self) -> Result<PendingBlockWithTxs, anyhow::Error> {
        match self.json_rpc_client.get_block_with_txs(BlockId::Tag(BlockTag::Pending)).await {
            Ok(MaybePendingBlockWithTxs::PendingBlock(b)) => Ok(b),
            other => Err(anyhow::format_err!("Got unexpected block: {other:?}")),
        }
    }

    pub async fn get_config(&self) -> serde_json::Value {
        self.send_custom_rpc("devnet_getConfig", json!({})).await.unwrap()
    }

    pub async fn execute_impersonation_action(
        &self,
        action: &ImpersonationAction,
    ) -> Result<(), anyhow::Error> {
        let (method_name, params) = match action {
            ImpersonationAction::ImpersonateAccount(account) => (
                "devnet_impersonateAccount",
                json!({
                    "account_address": to_hex_felt(account)
                }),
            ),
            ImpersonationAction::StopImpersonateAccount(account) => (
                "devnet_stopImpersonateAccount",
                json!({
                    "account_address": to_hex_felt(account)
                }),
            ),
            ImpersonationAction::AutoImpersonate => ("devnet_autoImpersonate", json!({})),
            ImpersonationAction::StopAutoImpersonate => ("devnet_stopAutoImpersonate", json!({})),
        };

        let result = self.send_custom_rpc(method_name, params).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(anyhow::Error::msg(err.message.to_string())),
        }
    }
}

/// By implementing Drop, we ensure there are no zombie background Devnet processes
/// in case of an early test failure
impl Drop for BackgroundDevnet {
    fn drop(&mut self) {
        self.process.kill().expect("Cannot kill process");
    }
}
