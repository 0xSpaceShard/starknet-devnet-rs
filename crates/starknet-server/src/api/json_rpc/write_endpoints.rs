use std::io::Read;

use base64::Engine;
use server::rpc_core::error::RpcError;
use starknet_types::contract_class::ContractClass;

use super::RpcResult;
use super::error::ApiError;
use crate::api::json_rpc::JsonRpcHandler;
use crate::api::models::contract_class::{DeprecatedContractClass};
use crate::api::models::transaction::{
    BroadcastedDeclareTransaction, BroadcastedDeclareTransactionV1,
};

impl JsonRpcHandler {
    pub(crate) async fn add_declare_transaction(
        &self,
        request: BroadcastedDeclareTransaction,
    ) -> RpcResult<()> {
        match request {
            BroadcastedDeclareTransaction::V1(BroadcastedDeclareTransactionV1 {
                common,
                contract_class,
                sender_address,
            }) => {

            }
            BroadcastedDeclareTransaction::V2(_) => todo!(),
        }

        Ok(())
    }

    fn convert_base64_gziped_json_string_to_json(json_str: &str) -> RpcResult<serde_json::Value> {
        let bytes = base64::engine::general_purpose::STANDARD.decode(json_str)
            .map_err(|_| ApiError::RpcError(RpcError::invalid_params(
                "program: Unable to decode base64 string"
            )))?;
        
        let mut decoder = flate2::read::GzDecoder::new(bytes.as_slice());
        let mut decoded = String::new();
        decoder.read_to_string(&mut decoded)
            .map_err(|_| ApiError::RpcError(RpcError::invalid_params("program: Unable to decode gzipped bytes")))?;

        let program_json = serde_json::from_str(&decoded).map_err(|_| {
            ApiError::RpcError(RpcError::invalid_params(
                "program: Unable to parse to JSON"
            ))
        })?;

        Ok(program_json)
    }
}

// impl TryFrom<DeprecatedContractClass> for ContractClass {
//     type Error = ApiError;

//     fn try_from(value: DeprecatedContractClass) -> Result<Self, Self::Error> {
//         let program_json = JsonRpcHandler::convert_base64_gziped_json_string_to_json(
//             &value.program
//         )?;

//         Ok(ContractClass {
//             address: value.address,
//             program: program_json,
//             abi: value.abi,
//             storage_layout: value.storage_layout,
//         })
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn convert_base64_gzipped_json_to_json_object_successfully() {
        let base64_gzipped_json = "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";

        super::JsonRpcHandler::convert_base64_gziped_json_string_to_json(base64_gzipped_json).unwrap();
    }
}