use thiserror::Error;
use {starknet_in_rust, starknet_types};

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    StarknetApiError(#[from] starknet_api::StarknetApiError),
    #[error(transparent)]
    StateError(#[from] starknet_in_rust::core::errors::state_errors::StateError),
    #[error(transparent)]
    TransactionError(#[from] starknet_in_rust::transaction::error::TransactionError),
    #[error("Types error")]
    TypesError(#[from] starknet_types::error::Error),
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
    #[error("Error when reading file {path}")]
    ReadFileError { source: std::io::Error, path: String },
    #[error("The file does not exist")]
    FileNotFound,
    #[error("Contract not found")]
    ContractNotFound,
    #[error(transparent)]
    SyscallHandlerError(
        #[from] starknet_in_rust::syscalls::syscall_handler_errors::SyscallHandlerError,
    ),
    #[error(transparent)]
    SignError(#[from] starknet_rs_signers::local_wallet::SignError),
    #[error("{msg}")]
    InvalidMintingTransaction { msg: String },
    #[error("No block found")]
    NoBlock,
    #[error("No state at block {block_number}")]
    NoStateAtBlock { block_number: u64 },
    #[error("Format error")]
    FormatError,
    #[error("Sierra compilation error")]
    SierraCompilationError,
    #[error("No transaction found")]
    NoTransaction,
    #[error("Invalid transaction index in a block")]
    InvalidTransactionIndexInBlock,
    #[error("{msg}")]
    UnsupportedAction { msg: String },
    #[error("Failed to load ContractClass")]
    ContractClassLoadError,
    #[error("Deserialization error of {obj_name}")]
    DeserializationError { obj_name: String },
    #[error("Serialization error of {obj_name}")]
    SerializationError { obj_name: String },
    #[error("Serialization not supported")]
    SerializationNotSupported,
}

pub type DevnetResult<T, E = Error> = Result<T, E>;
