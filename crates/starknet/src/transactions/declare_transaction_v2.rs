use starknet_in_rust::core::transaction_hash::{
    calculate_transaction_hash_common, TransactionHashPrefix,
};
use starknet_types::contract_address::ContractAddress;
use starknet_types::contract_class::ContractClass;
use starknet_types::felt::{ClassHash, Felt, TransactionHash};
use starknet_types::traits::HashProducer;
use starknet_types::DevnetResult;

use crate::error::{Error, Result};

#[derive(Clone, PartialEq, Eq)]
pub struct DeclareTransactionV2 {
    pub sierra_contract_class: ContractClass,
    pub compiled_class_hash: ClassHash,
    pub sender_address: ContractAddress,
    pub max_fee: u128,
    pub signature: Vec<Felt>,
    pub nonce: Felt,
    pub class_hash: Option<ClassHash>,
    pub transaction_hash: Option<TransactionHash>,
    pub chain_id: Felt,
}

impl DeclareTransactionV2 {
    pub fn new(
        sierra_contract_class: ContractClass,
        compiled_class_hash: ClassHash,
        sender_address: ContractAddress,
        max_fee: u128,
        signature: Vec<Felt>,
        nonce: Felt,
        chain_id: Felt,
    ) -> Result<Self> {
        if max_fee == 0 {
            return Err(Error::TransactionError(
                starknet_in_rust::transaction::error::TransactionError::FeeError(
                    "For declare transaction version 2, max fee cannot be 0".to_string(),
                ),
            ));
        }

        Ok(Self {
            sierra_contract_class,
            compiled_class_hash,
            sender_address,
            max_fee,
            signature,
            nonce,
            class_hash: None,
            transaction_hash: None,
            chain_id,
        })
    }
}

impl DeclareTransactionV2 {
    pub(crate) fn version(&self) -> Felt {
        Felt::from(2)
    }
}

impl HashProducer for DeclareTransactionV2 {
    fn generate_hash(&self) -> DevnetResult<Felt> {
        let class_hash = self.class_hash.unwrap_or(self.sierra_contract_class.generate_hash()?);

        let calldata = [class_hash.into()].to_vec();
        let additional_data = [self.nonce.into(), self.compiled_class_hash.into()].to_vec();

        let transaction_hash: Felt = calculate_transaction_hash_common(
            TransactionHashPrefix::Declare,
            self.version().into(),
            &self.sender_address.try_into()?,
            Felt::from(0).into(),
            &calldata,
            self.max_fee,
            self.chain_id.into(),
            &additional_data,
        )
        .map_err(|err| {
            starknet_types::error::Error::TransactionError(
                starknet_in_rust::transaction::error::TransactionError::Syscall(err),
            )
        })?
        .into();

        Ok(transaction_hash)
    }
}

#[cfg(test)]
mod tests {
    use starknet_types::contract_class::ContractClass;


    #[ignore]
    #[test]
    fn correct_declare_transaction_hash_computation() {
        todo!("Transaction hash computation should be checked")
    }
}
