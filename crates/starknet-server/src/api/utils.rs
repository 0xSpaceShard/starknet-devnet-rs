use starknet_types::starknet_api::transaction::Fee;

use super::json_rpc::error::ApiError;
use super::models::transaction::{
    DeclareTransaction, DeclareTransactionV0V1, DeclareTransactionV2, DeployAccountTransaction,
    InvokeTransactionV1, Transaction, TransactionType, TransactionWithType, TransactionReceiptWithStatus, CommonTransactionReceipt,
};

impl TryFrom<&starknet_core::transactions::StarknetTransaction> for TransactionReceiptWithStatus {
    type Error = ApiError;

    fn try_from(txn: &starknet_core::transactions::StarknetTransaction) -> Result<Self, Self::Error> {

        let transaction_with_receipt = match txn.inner {
            starknet_core::transactions::Transaction::Declare(declare_v1) => {
                let out = crate::api::models::transaction::TransactionOutput {
                    actual_fee: types::starknet_api::transaction::Fee(txn.max_fee()),
                    messages_sent: Vec::new(), // where to find it?
                    events: Vec::new(), // where to find it?
                };
        
                let receipt = crate::api::models::transaction::TransactionReceipt::Common(CommonTransactionReceipt{
                    r#type: crate::api::models::transaction::TransactionType::Declare,
                    transaction_hash: txn.inner.get_hash(),
                    block_hash: txn.block_hash.unwrap(),
                    block_number: txn.block_number.unwrap(),
                    output: out,
                });
                
                TransactionReceiptWithStatus {
                    status: crate::api::models::transaction::TransactionStatus::AcceptedOnL2, //todo map txn.status
                    receipt: receipt,
                }
            }
            starknet_core::transactions::Transaction::DeclareV2(declare_v2) => {

                TransactionReceiptWithStatus {
                    status: crate::api::models::transaction::TransactionStatus::AcceptedOnL2, //todo map txn.status
                    receipt: receipt,
                }
            }
            starknet_core::transactions::Transaction::DeployAccount(deploy_account) => {

                TransactionReceiptWithStatus {
                    status: crate::api::models::transaction::TransactionStatus::AcceptedOnL2, //todo map txn.status
                    receipt: receipt,
                }
            }
            starknet_core::transactions::Transaction::Invoke(invoke_v1) => {

                TransactionReceiptWithStatus {
                    status: crate::api::models::transaction::TransactionStatus::AcceptedOnL2, //todo map txn.status
                    receipt: receipt,
                }
            }
        };

        Ok(transaction_with_receipt)
    }
}


impl TryFrom<&starknet_core::transactions::Transaction> for TransactionWithType {
    type Error = ApiError;
    fn try_from(txn: &starknet_core::transactions::Transaction) -> Result<Self, Self::Error> {
        let transaction_with_type = match txn {
            starknet_core::transactions::Transaction::Declare(declare_v1) => {
                let declare_txn = DeclareTransactionV0V1 {
                    class_hash: *declare_v1.class_hash(),
                    sender_address: *declare_v1.sender_address(),
                    nonce: *txn.nonce(),
                    max_fee: Fee(txn.max_fee()),
                    version: *txn.version(),
                    transaction_hash: txn.get_hash(),
                    signature: txn.signature().to_vec(),
                };
                TransactionWithType {
                    r#type: TransactionType::Declare,
                    transaction: Transaction::Declare(DeclareTransaction::Version1(declare_txn)),
                }
            }
            starknet_core::transactions::Transaction::DeclareV2(declare_v2) => {
                let declare_txn = DeclareTransactionV2 {
                    class_hash: *declare_v2.class_hash(),
                    compiled_class_hash: *declare_v2.compiled_class_hash(),
                    sender_address: *declare_v2.sender_address(),
                    nonce: *txn.nonce(),
                    max_fee: Fee(txn.max_fee()),
                    version: *txn.version(),
                    transaction_hash: txn.get_hash(),
                    signature: txn.signature().to_vec(),
                };

                TransactionWithType {
                    r#type: TransactionType::Declare,
                    transaction: Transaction::Declare(DeclareTransaction::Version2(declare_txn)),
                }
            }
            starknet_core::transactions::Transaction::DeployAccount(deploy_account) => {
                let deploy_account_txn = DeployAccountTransaction {
                    nonce: *txn.nonce(),
                    max_fee: Fee(txn.max_fee()),
                    version: *txn.version(),
                    transaction_hash: txn.get_hash(),
                    signature: txn.signature().to_vec(),
                    class_hash: deploy_account
                        .class_hash()
                        .map_err(ApiError::StarknetDevnetError)?,
                    contract_address_salt: deploy_account.contract_address_salt(),
                    constructor_calldata: deploy_account.constructor_calldata(),
                };

                TransactionWithType {
                    r#type: TransactionType::DeployAccount,
                    transaction: Transaction::DeployAccount(deploy_account_txn),
                }
            }
            starknet_core::transactions::Transaction::Invoke(invoke_v1) => {
                let invoke_txn = InvokeTransactionV1 {
                    sender_address: invoke_v1
                        .sender_address()
                        .map_err(ApiError::StarknetDevnetError)?,
                    nonce: *txn.nonce(),
                    max_fee: Fee(txn.max_fee()),
                    version: *txn.version(),
                    transaction_hash: txn.get_hash(),
                    signature: txn.signature().to_vec(),
                    calldata: invoke_v1.calldata().to_vec(),
                };

                TransactionWithType {
                    r#type: TransactionType::Invoke,
                    transaction: Transaction::Invoke(
                        crate::api::models::transaction::InvokeTransaction::Version1(invoke_txn),
                    ),
                }
            }
        };

        Ok(transaction_with_type)
    }
}
