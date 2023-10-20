use std::fmt;
use std::net::IpAddr;

use starknet_types::chain_id::ChainId;
use starknet_types::felt::Felt;
use strum_macros::EnumIter;

use crate::constants::{DEVNET_DEFAULT_CHAIN_ID, DEVNET_DEFAULT_HOST};

#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumIter)]
pub enum DumpMode {
    OnExit,
    OnTransaction,
}

impl fmt::Display for DumpMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            DumpMode::OnExit => write!(f, "exit"),
            DumpMode::OnTransaction => write!(f, "transaction"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StarknetConfig {
    pub seed: u32,
    pub total_accounts: u8,
    pub predeployed_accounts_initial_balance: Felt,
    pub host: IpAddr,
    pub port: u16,
    pub timeout: u16,
    pub gas_price: u64,
    pub chain_id: ChainId,
    pub dump_on: Option<DumpMode>,
    pub dump_path: Option<String>,
}

impl Default for StarknetConfig {
    fn default() -> Self {
        Self {
            seed: u32::default(),
            total_accounts: u8::default(),
            predeployed_accounts_initial_balance: Felt::default(),
            host: DEVNET_DEFAULT_HOST,
            port: u16::default(),
            timeout: u16::default(),
            gas_price: Default::default(),
            chain_id: DEVNET_DEFAULT_CHAIN_ID,
            dump_on: None,
            dump_path: None,
        }
    }
}
