use serde::{Deserialize, Serialize};
use starknet_api::block::BlockNumber;
use starknet_api::core::EthAddress;
use starknet_api::transaction::Fee;
use starknet_rs_core::types::{ExecutionResult, TransactionFinalityStatus};

use crate::constants::{
    BITWISE_BUILTIN_NAME, EC_OP_BUILTIN_NAME, HASH_BUILTIN_NAME, KECCAK_BUILTIN_NAME, N_STEPS,
    POSEIDON_BUILTIN_NAME, RANGE_CHECK_BUILTIN_NAME, SIGNATURE_BUILTIN_NAME,
};
use crate::contract_address::ContractAddress;
use crate::emitted_event::Event;
use crate::felt::{BlockHash, Felt, TransactionHash};
use crate::rpc::transactions::TransactionType;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionReceipt {
    Deploy(DeployTransactionReceipt),
    Common(CommonTransactionReceipt),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeployTransactionReceipt {
    #[serde(flatten)]
    pub common: CommonTransactionReceipt,
    pub contract_address: ContractAddress,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct MaybePendingProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_hash: Option<BlockHash>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_number: Option<BlockNumber>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommonTransactionReceipt {
    pub r#type: TransactionType,
    pub transaction_hash: TransactionHash,
    pub actual_fee: FeeInUnits,
    pub messages_sent: Vec<MessageToL1>,
    pub events: Vec<Event>,
    #[serde(flatten)]
    pub execution_status: ExecutionResult,
    pub finality_status: TransactionFinalityStatus,
    #[serde(flatten)]
    pub maybe_pending_properties: MaybePendingProperties,
    pub execution_resources: ExecutionResources,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExecutionResources {
    pub steps: usize,
    pub memory_holes: Option<usize>,
    pub range_check_builtin_applications: Option<usize>,
    pub pedersen_builtin_applications: Option<usize>,
    pub poseidon_builtin_applications: Option<usize>,
    pub ec_op_builtin_applications: Option<usize>,
    pub ecdsa_builtin_applications: Option<usize>,
    pub bitwise_builtin_applications: Option<usize>,
    pub keccak_builtin_applications: Option<usize>,
}

impl From<&blockifier::execution::call_info::CallInfo> for ExecutionResources {
    fn from(call_info: &blockifier::execution::call_info::CallInfo) -> Self {
        ExecutionResources {
            steps: call_info.vm_resources.n_steps,
            memory_holes: if call_info.vm_resources.n_memory_holes == 0 {
                None
            } else {
                Some(call_info.vm_resources.n_memory_holes)
            },
            range_check_builtin_applications: Self::get_resource_from_call_info(
                call_info,
                RANGE_CHECK_BUILTIN_NAME,
            ),
            pedersen_builtin_applications: Self::get_resource_from_call_info(
                call_info,
                HASH_BUILTIN_NAME,
            ),
            poseidon_builtin_applications: Self::get_resource_from_call_info(
                call_info,
                POSEIDON_BUILTIN_NAME,
            ),
            ec_op_builtin_applications: Self::get_resource_from_call_info(
                call_info,
                EC_OP_BUILTIN_NAME,
            ),
            ecdsa_builtin_applications: Self::get_resource_from_call_info(
                call_info,
                SIGNATURE_BUILTIN_NAME,
            ),
            bitwise_builtin_applications: Self::get_resource_from_call_info(
                call_info,
                BITWISE_BUILTIN_NAME,
            ),
            keccak_builtin_applications: Self::get_resource_from_call_info(
                call_info,
                KECCAK_BUILTIN_NAME,
            ),
        }
    }
}

impl From<&blockifier::transaction::objects::TransactionExecutionInfo> for ExecutionResources {
    fn from(execution_info: &blockifier::transaction::objects::TransactionExecutionInfo) -> Self {
        let total_memory_holes =
            Self::get_memory_holes_from_call_info(&execution_info.execute_call_info)
                + Self::get_memory_holes_from_call_info(&execution_info.validate_call_info)
                + Self::get_memory_holes_from_call_info(&execution_info.fee_transfer_call_info);

        Self {
            steps: Self::get_resource_from_execution_info(execution_info, N_STEPS)
                .unwrap_or_default(),
            memory_holes: if total_memory_holes == 0 { None } else { Some(total_memory_holes) },
            range_check_builtin_applications: Self::get_resource_from_execution_info(
                execution_info,
                RANGE_CHECK_BUILTIN_NAME,
            ),
            pedersen_builtin_applications: Self::get_resource_from_execution_info(
                execution_info,
                HASH_BUILTIN_NAME,
            ),
            poseidon_builtin_applications: Self::get_resource_from_execution_info(
                execution_info,
                POSEIDON_BUILTIN_NAME,
            ),
            ec_op_builtin_applications: Self::get_resource_from_execution_info(
                execution_info,
                EC_OP_BUILTIN_NAME,
            ),
            ecdsa_builtin_applications: Self::get_resource_from_execution_info(
                execution_info,
                SIGNATURE_BUILTIN_NAME,
            ),
            bitwise_builtin_applications: Self::get_resource_from_execution_info(
                execution_info,
                BITWISE_BUILTIN_NAME,
            ),
            keccak_builtin_applications: Self::get_resource_from_execution_info(
                execution_info,
                KECCAK_BUILTIN_NAME,
            ),
        }
    }
}

impl ExecutionResources {
    fn get_memory_holes_from_call_info(
        call_info: &Option<blockifier::execution::call_info::CallInfo>,
    ) -> usize {
        if let Some(call) = call_info { call.vm_resources.n_memory_holes } else { 0 }
    }

    fn get_resource_from_execution_info(
        execution_info: &blockifier::transaction::objects::TransactionExecutionInfo,
        resource_name: &str,
    ) -> Option<usize> {
        execution_info.actual_resources.0.get(resource_name).cloned()
    }

    fn get_resource_from_call_info(
        call_info: &blockifier::execution::call_info::CallInfo,
        resource_name: &str,
    ) -> Option<usize> {
        call_info.vm_resources.builtin_instance_counter.get(resource_name).cloned()
    }
}

impl PartialEq for CommonTransactionReceipt {
    fn eq(&self, other: &Self) -> bool {
        let identical_execution_result = match (&self.execution_status, &other.execution_status) {
            (ExecutionResult::Succeeded, ExecutionResult::Succeeded) => true,
            (
                ExecutionResult::Reverted { reason: reason1 },
                ExecutionResult::Reverted { reason: reason2 },
            ) => reason1 == reason2,
            _ => false,
        };

        self.transaction_hash == other.transaction_hash
            && self.r#type == other.r#type
            && self.maybe_pending_properties == other.maybe_pending_properties
            && self.events == other.events
            && self.messages_sent == other.messages_sent
            && self.actual_fee == other.actual_fee
            && self.execution_resources == other.execution_resources
            && identical_execution_result
    }
}

impl Eq for CommonTransactionReceipt {}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FeeAmount {
    pub amount: Fee,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "unit")]
pub enum FeeInUnits {
    #[serde(rename = "WEI")]
    WEI(FeeAmount),
    #[serde(rename = "FRI")]
    STRK(FeeAmount),
}

pub type L2ToL1Payload = Vec<Felt>;

/// An L2 to L1 message.
#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MessageToL1 {
    pub from_address: ContractAddress,
    pub to_address: EthAddress,
    pub payload: L2ToL1Payload,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrderedMessageToL1 {
    pub order: usize,
    #[serde(flatten)]
    pub message: MessageToL1,
}

impl OrderedMessageToL1 {
    pub fn new(
        msg: blockifier::execution::call_info::OrderedL2ToL1Message,
        from_address: ContractAddress,
    ) -> Self {
        Self {
            order: msg.order,
            message: MessageToL1 {
                from_address,
                to_address: msg.message.to_address,
                payload: msg.message.payload.0.into_iter().map(Felt::from).collect(),
            },
        }
    }
}
