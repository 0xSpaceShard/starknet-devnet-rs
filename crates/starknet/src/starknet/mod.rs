use std::fmt;
use std::net::IpAddr;
use std::time::SystemTime;

use blockifier::block_context::BlockContext;
use blockifier::execution::entry_point::CallEntryPoint;
use blockifier::state::state_api::StateReader;
use blockifier::transaction::objects::TransactionExecutionInfo;
use blockifier::transaction::transactions::ExecutableTransaction;
use starknet_api::block::{BlockNumber, BlockStatus, BlockTimestamp, GasPrice};
use starknet_api::transaction::Fee;
use starknet_in_rust::definitions::constants::DEFAULT_CAIRO_RESOURCE_FEE_WEIGHTS;
use starknet_rs_core::types::{BlockId, MsgFromL1, TransactionFinalityStatus};
use starknet_rs_core::utils::get_selector_from_name;
use starknet_rs_ff::FieldElement;
use starknet_rs_signers::Signer;
use starknet_types::chain_id::ChainId;
use starknet_types::contract_address::ContractAddress;
use starknet_types::contract_class::{Cairo0Json, ContractClass};
use starknet_types::contract_storage_key::ContractStorageKey;
use starknet_types::emitted_event::EmittedEvent;
use starknet_types::felt::{ClassHash, Felt, TransactionHash};
use starknet_types::patricia_key::PatriciaKey;
use starknet_types::rpc::block::{Block, BlockHeader};
use starknet_types::rpc::estimate_message_fee::FeeEstimateWrapper;
use starknet_types::rpc::transaction_receipt::TransactionReceipt;
use starknet_types::rpc::transactions::broadcasted_declare_transaction_v1::BroadcastedDeclareTransactionV1;
use starknet_types::rpc::transactions::broadcasted_declare_transaction_v2::BroadcastedDeclareTransactionV2;
use starknet_types::rpc::transactions::broadcasted_deploy_account_transaction::BroadcastedDeployAccountTransaction;
use starknet_types::rpc::transactions::broadcasted_invoke_transaction::BroadcastedInvokeTransaction;
use starknet_types::rpc::transactions::{
    BroadcastedTransaction, BroadcastedTransactionCommon, DeclareTransaction,
    DeclareTransactionTrace, DeployAccountTransactionTrace, ExecutionInvocation,
    FunctionInvocation, InvokeTransactionTrace, SimulatedTransaction, SimulationFlag, Transaction,
    TransactionTrace, Transactions,
};
use starknet_types::traits::HashProducer;
use strum_macros::EnumIter;
use tracing::{error, warn};

use self::predeployed::initialize_erc20;
use crate::account::Account;
use crate::blocks::{StarknetBlock, StarknetBlocks};
use crate::constants::{
    CAIRO_0_ACCOUNT_CONTRACT_PATH, CHARGEABLE_ACCOUNT_ADDRESS, CHARGEABLE_ACCOUNT_PRIVATE_KEY,
    DEVNET_DEFAULT_CHAIN_ID, DEVNET_DEFAULT_HOST, ERC20_CONTRACT_ADDRESS,
};
use crate::error::{DevnetResult, Error, TransactionValidationError};
use crate::predeployed_accounts::PredeployedAccounts;
use crate::raw_execution::{Call, RawExecution};
use crate::state::state_diff::StateDiff;
use crate::state::state_update::StateUpdate;
use crate::state::StarknetState;
use crate::traits::{
    AccountGenerator, Accounted, Deployed, HashIdentified, HashIdentifiedMut, StateChanger,
    StateExtractor,
};
use crate::transactions::{StarknetTransaction, StarknetTransactions};

mod add_declare_transaction;
mod add_deploy_account_transaction;
mod add_invoke_transaction;
mod dump;
mod estimations;
mod events;
mod get_class_impls;
mod predeployed;
mod state_update;

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

pub struct Starknet {
    pub(in crate::starknet) state: StarknetState,
    predeployed_accounts: PredeployedAccounts,
    pub(in crate::starknet) block_context: BlockContext,
    blocks: StarknetBlocks,
    pub transactions: StarknetTransactions,
    pub config: StarknetConfig,
}

impl Default for Starknet {
    fn default() -> Self {
        Self {
            block_context: Self::init_block_context(
                0,
                ERC20_CONTRACT_ADDRESS,
                DEVNET_DEFAULT_CHAIN_ID,
            ),
            state: Default::default(),
            predeployed_accounts: Default::default(),
            blocks: Default::default(),
            transactions: Default::default(),
            config: Default::default(),
        }
    }
}

impl Starknet {
    pub fn new(config: &StarknetConfig) -> DevnetResult<Self> {
        let mut state = StarknetState::default();
        // deploy udc and erc20 contracts
        let erc20_fee_contract = predeployed::create_erc20()?;
        let udc_contract = predeployed::create_udc()?;

        erc20_fee_contract.deploy(&mut state)?;
        initialize_erc20(&mut state)?;

        udc_contract.deploy(&mut state)?;

        let mut predeployed_accounts = PredeployedAccounts::new(
            config.seed,
            config.predeployed_accounts_initial_balance,
            erc20_fee_contract.get_address(),
        );
        let account_contract_class = Cairo0Json::raw_json_from_path(CAIRO_0_ACCOUNT_CONTRACT_PATH)?;
        let class_hash = account_contract_class.generate_hash()?;

        let accounts = predeployed_accounts.generate_accounts(
            config.total_accounts,
            class_hash,
            account_contract_class.clone().into(),
        )?;
        for account in accounts {
            account.deploy(&mut state)?;
            account.set_initial_balance(&mut state)?;
        }

        let chargeable_account = Account::new_chargeable(
            class_hash,
            account_contract_class.into(),
            erc20_fee_contract.get_address(),
        );
        chargeable_account.deploy(&mut state)?;
        chargeable_account.set_initial_balance(&mut state)?;

        // copy already modified state to cached state
        state.clear_dirty_state();

        let mut this = Self {
            state,
            predeployed_accounts,
            block_context: Self::init_block_context(
                config.gas_price,
                ERC20_CONTRACT_ADDRESS,
                config.chain_id,
            ),
            blocks: StarknetBlocks::default(),
            transactions: StarknetTransactions::default(),
            config: config.clone(),
        };

        this.restart_pending_block()?;

        // Load starknet transactions
        if this.config.dump_path.is_some() {
            let transactions = this.load_transactions()?;
            this.re_execute(transactions)?;
        }

        Ok(this)
    }

    pub fn get_predeployed_accounts(&self) -> Vec<Account> {
        self.predeployed_accounts.get_accounts().to_vec()
    }

    // Update block context
    // Initialize values for new pending block
    pub(crate) fn generate_pending_block(&mut self) -> DevnetResult<()> {
        Self::update_block_context(&mut self.block_context);
        self.restart_pending_block()?;

        Ok(())
    }

    /// Transfer data from pending block into new block and save it to blocks collection
    /// Returns the new block number
    pub(crate) fn generate_new_block(
        &mut self,
        state_diff: StateDiff,
    ) -> DevnetResult<BlockNumber> {
        let mut new_block = self.pending_block().clone();

        // set new block header
        new_block.set_block_hash(new_block.generate_hash()?);
        new_block.status = BlockStatus::AcceptedOnL2;
        let new_block_number = new_block.block_number();

        // update txs block hash block number for each transaction in the pending block
        new_block.get_transactions().iter().for_each(|tx_hash| {
            if let Some(tx) = self.transactions.get_by_hash_mut(tx_hash) {
                tx.block_hash = Some(new_block.header.block_hash.0.into());
                tx.block_number = Some(new_block_number);
                tx.finality_status = Some(TransactionFinalityStatus::AcceptedOnL2);
            } else {
                error!("Transaction is not present in the transactions collection");
            }
        });

        // insert pending block in the blocks collection and connect it to the state diff
        self.blocks.insert(new_block, state_diff);
        // save into blocks state archive

        let deep_cloned_state = self.state.clone();
        self.blocks.save_state_at(new_block_number, deep_cloned_state);

        Ok(new_block_number)
    }

    /// Handles transaction result either Ok or Error and updates the state accordingly.
    ///
    /// # Arguments
    ///
    /// * `transaction` - Transaction to be added in the collection of transactions.
    /// * `transaction_result` - Result with transaction_execution_info
    pub(crate) fn handle_transaction_result(
        &mut self,
        transaction: Transaction,
        transaction_result: Result<
            TransactionExecutionInfo,
            blockifier::transaction::errors::TransactionExecutionError,
        >,
    ) -> DevnetResult<()> {
        let transaction_hash = *transaction.get_transaction_hash();

        match transaction_result {
            Ok(tx_info) => {
                // If transaction is not reverted
                // then save the contract class in the state cache for Declare V1/V2 transactions
                if !tx_info.is_reverted() {
                    match &transaction {
                        Transaction::Declare(DeclareTransaction::Version1(declare_v1)) => {
                            self.state.contract_classes.insert(
                                declare_v1.class_hash,
                                declare_v1.contract_class.clone().into(),
                            );
                        }
                        Transaction::Declare(DeclareTransaction::Version2(declare_v2)) => {
                            self.state.contract_classes.insert(
                                declare_v2.class_hash,
                                declare_v2.contract_class.clone().into(),
                            );
                        }
                        _ => {}
                    };
                }
                self.handle_accepted_transaction(&transaction_hash, &transaction, tx_info)
            }
            Err(tx_err) => {
                // based on this https://community.starknet.io/t/efficient-utilization-of-sequencer-capacity-in-starknet-v0-12-1/95607#the-validation-phase-in-the-gateway-5
                // we should not save transactions that failed with one of the following errors
                match tx_err {
                    blockifier::transaction::errors::TransactionExecutionError::InvalidNonce { .. } =>
                        Err(TransactionValidationError::InvalidTransactionNonce.into()),
                    blockifier::transaction::errors::TransactionExecutionError::MaxFeeExceedsBalance { .. } =>
                        Err(TransactionValidationError::InsufficientAccountBalance.into()),
                    blockifier::transaction::errors::TransactionExecutionError::MaxFeeTooLow { .. } =>
                        Err(TransactionValidationError::InsufficientMaxFee.into()),
                    blockifier::transaction::errors::TransactionExecutionError::ValidateTransactionError(..) =>
                        Err(TransactionValidationError::GeneralFailure.into()),
                    _ => Err(tx_err.into())
                }
            }
        }
    }

    /// Handles suceeded and reverted transactions.
    /// The tx is stored and potentially dumped.
    /// A new block is generated.
    pub(crate) fn handle_accepted_transaction(
        &mut self,
        transaction_hash: &TransactionHash,
        transaction: &Transaction,
        tx_info: TransactionExecutionInfo,
    ) -> DevnetResult<()> {
        let transaction_to_add = StarknetTransaction::create_accepted(transaction, tx_info);

        // add accepted transaction to pending block
        self.blocks.pending_block.add_transaction(*transaction_hash);

        self.transactions.insert(transaction_hash, transaction_to_add);

        let state_difference = self.state.extract_state_diff_from_pending_state()?;
        // apply state changes from cached state
        self.state.apply_state_difference(state_difference.clone())?;
        // make cached state part of "persistent" state
        self.state.clear_dirty_state();
        // create new block from pending one
        self.generate_new_block(state_difference)?;
        // clear pending block information
        self.generate_pending_block()?;

        if self.config.dump_on == Some(DumpMode::OnTransaction) {
            self.dump_transaction(transaction)?;
        }

        Ok(())
    }

    fn init_block_context(
        gas_price: u64,
        fee_token_address: &str,
        chain_id: ChainId,
    ) -> BlockContext {
        use starknet_api::core::{ContractAddress, PatriciaKey};
        use starknet_api::hash::StarkHash;
        use starknet_api::{contract_address, patricia_key};

        let mut block_context = blockifier::block_context::BlockContext::create_for_testing();

        block_context.block_number = BlockNumber(0);
        block_context.block_timestamp = BlockTimestamp(0);
        block_context.gas_price = gas_price as u128;
        block_context.chain_id = chain_id.into();
        block_context.fee_token_address = contract_address!(fee_token_address);
        // inject cairo resource fee weights from starknet_in_rust
        block_context.vm_resource_fee_cost =
            std::sync::Arc::new(DEFAULT_CAIRO_RESOURCE_FEE_WEIGHTS.clone());

        block_context
    }

    /// Should update block context with new block timestamp
    /// and pointer to the next block number
    fn update_block_context(block_context: &mut BlockContext) {
        let current_timestamp_secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("should get current UNIX timestamp")
            .as_secs();

        block_context.block_number = block_context.block_number.next();
        block_context.block_timestamp = BlockTimestamp(current_timestamp_secs);
    }

    fn pending_block(&self) -> &StarknetBlock {
        &self.blocks.pending_block
    }

    /// Restarts pending block with information from block_context
    fn restart_pending_block(&mut self) -> DevnetResult<()> {
        let mut block = StarknetBlock::create_pending_block();

        block.header.block_number = self.block_context.block_number;
        block.header.gas_price = GasPrice(self.block_context.gas_price);
        block.header.sequencer = self.block_context.sequencer_address;
        block.header.timestamp = self.block_context.block_timestamp;

        self.blocks.pending_block = block;

        Ok(())
    }

    fn get_state_at(&self, block_id: &BlockId) -> DevnetResult<&StarknetState> {
        match block_id {
            BlockId::Tag(_) => Ok(&self.state),
            _ => {
                let block = self.blocks.get_by_block_id(*block_id).ok_or(Error::NoBlock)?;
                let state = self
                    .blocks
                    .num_to_state
                    .get(&block.block_number())
                    .ok_or(Error::NoStateAtBlock { block_number: block.block_number().0 })?;
                Ok(state)
            }
        }
    }

    pub fn get_class_hash_at(
        &self,
        block_id: BlockId,
        contract_address: ContractAddress,
    ) -> DevnetResult<ClassHash> {
        get_class_impls::get_class_hash_at_impl(self, block_id, contract_address)
    }

    pub fn get_class(
        &self,
        block_id: BlockId,
        class_hash: ClassHash,
    ) -> DevnetResult<ContractClass> {
        get_class_impls::get_class_impl(self, block_id, class_hash)
    }

    pub fn get_class_at(
        &self,
        block_id: BlockId,
        contract_address: ContractAddress,
    ) -> DevnetResult<ContractClass> {
        get_class_impls::get_class_at_impl(self, block_id, contract_address)
    }

    pub fn call(
        &self,
        block_id: BlockId,
        contract_address: Felt,
        entrypoint_selector: Felt,
        calldata: Vec<Felt>,
    ) -> DevnetResult<Vec<Felt>> {
        let state = self.get_state_at(&block_id)?;

        if !self.state.is_contract_deployed(&ContractAddress::new(contract_address)?) {
            return Err(Error::ContractNotFound);
        }

        let call = CallEntryPoint {
            calldata: starknet_api::transaction::Calldata(std::sync::Arc::new(
                calldata.iter().map(|f| f.into()).collect(),
            )),
            storage_address: starknet_api::hash::StarkFelt::from(contract_address).try_into()?,
            entry_point_selector: starknet_api::core::EntryPointSelector(
                entrypoint_selector.into(),
            ),
            initial_gas: blockifier::transaction::transaction_execution::Transaction::initial_gas(),
            ..Default::default()
        };

        let res = call.execute(
            &mut state.clone().state,
            &mut blockifier::execution::entry_point::ExecutionResources::default(),
            &mut blockifier::execution::entry_point::EntryPointExecutionContext::new(
                self.block_context.clone(),
                blockifier::transaction::objects::AccountTransactionContext::default(),
                self.block_context.invoke_tx_max_n_steps as usize,
            ),
        ).map_err(|err| Error::BlockifierTransactionError(blockifier::transaction::errors::TransactionExecutionError::EntryPointExecutionError(err)))?;

        Ok(res.execution.retdata.0.into_iter().map(Felt::from).collect())
    }

    pub fn estimate_fee(
        &self,
        block_id: BlockId,
        transactions: &[BroadcastedTransaction],
    ) -> DevnetResult<Vec<FeeEstimateWrapper>> {
        estimations::estimate_fee(self, block_id, transactions)
    }

    pub fn estimate_message_fee(
        &self,
        block_id: BlockId,
        message: MsgFromL1,
    ) -> DevnetResult<FeeEstimateWrapper> {
        estimations::estimate_message_fee(self, block_id, message)
    }

    pub fn add_declare_transaction_v1(
        &mut self,
        declare_transaction: BroadcastedDeclareTransactionV1,
    ) -> DevnetResult<(TransactionHash, ClassHash)> {
        add_declare_transaction::add_declare_transaction_v1(self, declare_transaction)
    }

    pub fn add_declare_transaction_v2(
        &mut self,
        declare_transaction: BroadcastedDeclareTransactionV2,
    ) -> DevnetResult<(TransactionHash, ClassHash)> {
        add_declare_transaction::add_declare_transaction_v2(self, declare_transaction)
    }

    /// returning the chain id as object
    pub fn chain_id(&self) -> ChainId {
        self.config.chain_id
    }

    pub fn add_deploy_account_transaction(
        &mut self,
        deploy_account_transaction: BroadcastedDeployAccountTransaction,
    ) -> DevnetResult<(TransactionHash, ContractAddress)> {
        add_deploy_account_transaction::add_deploy_account_transaction(
            self,
            deploy_account_transaction,
        )
    }

    pub fn add_invoke_transaction(
        &mut self,
        invoke_transaction: BroadcastedInvokeTransaction,
    ) -> DevnetResult<TransactionHash> {
        add_invoke_transaction::add_invoke_transaction(self, invoke_transaction)
    }

    /// Creates an invoke tx for minting, using the chargeable account.
    pub async fn mint(&mut self, address: ContractAddress, amount: u128) -> DevnetResult<Felt> {
        let sufficiently_big_max_fee: u128 = self.config.gas_price as u128 * 1_000_000;
        let chargeable_address_felt = Felt::from_prefixed_hex_str(CHARGEABLE_ACCOUNT_ADDRESS)?;
        let nonce =
            self.state.state.get_nonce_at(starknet_api::core::ContractAddress::try_from(
                starknet_api::hash::StarkFelt::from(chargeable_address_felt),
            )?)?;

        let calldata = vec![
            Felt::from(address).into(),
            FieldElement::from(amount), // `low` part of Uint256
            FieldElement::from(0u32),   // `high` part
        ];

        let erc20_address_felt = Felt::from_prefixed_hex_str(ERC20_CONTRACT_ADDRESS)?;
        let raw_execution = RawExecution {
            calls: vec![Call {
                to: erc20_address_felt.into(),
                selector: get_selector_from_name("mint").unwrap(),
                calldata: calldata.clone(),
            }],
            nonce: Felt::from(nonce.0).into(),
            max_fee: FieldElement::from(sufficiently_big_max_fee),
        };

        // generate msg hash (not the same as tx hash)
        let chain_id_felt: Felt = self.config.chain_id.to_felt();
        let msg_hash_felt =
            raw_execution.transaction_hash(chain_id_felt.into(), chargeable_address_felt.into());

        // generate signature by signing the msg hash
        let signer = starknet_rs_signers::LocalWallet::from(
            starknet_rs_signers::SigningKey::from_secret_scalar(
                FieldElement::from_hex_be(CHARGEABLE_ACCOUNT_PRIVATE_KEY).unwrap(),
            ),
        );
        let signature = signer.sign_hash(&msg_hash_felt).await?;

        let invoke_tx = BroadcastedInvokeTransaction {
            sender_address: ContractAddress::new(chargeable_address_felt)?,
            calldata: raw_execution.raw_calldata().into_iter().map(|c| c.into()).collect(),
            common: BroadcastedTransactionCommon {
                max_fee: Fee(sufficiently_big_max_fee),
                version: Felt::from(1),
                signature: vec![signature.r.into(), signature.s.into()],
                nonce: nonce.0.into(),
            },
        };

        // apply the invoke tx
        self.add_invoke_transaction(invoke_tx)
    }

    pub fn block_state_update(&self, block_id: BlockId) -> DevnetResult<StateUpdate> {
        state_update::state_update_by_block_id(self, block_id)
    }

    pub fn get_block_txs_count(&self, block_id: BlockId) -> DevnetResult<u64> {
        let block = self.blocks.get_by_block_id(block_id).ok_or(Error::NoBlock)?;

        Ok(block.get_transactions().len() as u64)
    }

    pub fn contract_nonce_at_block(
        &self,
        block_id: BlockId,
        contract_address: ContractAddress,
    ) -> DevnetResult<Felt> {
        let state = self.get_state_at(&block_id)?;
        state.get_nonce(&contract_address)
    }

    pub fn contract_storage_at_block(
        &self,
        block_id: BlockId,
        contract_address: ContractAddress,
        storage_key: PatriciaKey,
    ) -> DevnetResult<Felt> {
        let state = self.get_state_at(&block_id)?;
        state.get_storage(ContractStorageKey::new(contract_address, storage_key))
    }

    pub fn get_block(&self, block_id: BlockId) -> DevnetResult<StarknetBlock> {
        let block = self.blocks.get_by_block_id(block_id).ok_or(Error::NoBlock)?;
        Ok(block.clone())
    }

    pub fn get_block_with_transactions(&self, block_id: BlockId) -> DevnetResult<Block> {
        let block = self.blocks.get_by_block_id(block_id).ok_or(Error::NoBlock)?;
        let transactions = block
            .get_transactions()
            .iter()
            .map(|transaction_hash| {
                self.transactions
                    .get_by_hash(*transaction_hash)
                    .ok_or(Error::NoTransaction)
                    .map(|transaction| transaction.inner.clone())
            })
            .collect::<DevnetResult<Vec<Transaction>>>()?;

        Ok(Block {
            status: *block.status(),
            header: BlockHeader::from(block),
            transactions: Transactions::Full(transactions),
        })
    }

    pub fn get_transaction_by_block_id_and_index(
        &self,
        block_id: BlockId,
        index: u64,
    ) -> DevnetResult<&Transaction> {
        let block = self.get_block(block_id)?;
        let transaction_hash = block
            .get_transactions()
            .get(index as usize)
            .ok_or(Error::InvalidTransactionIndexInBlock)?;

        self.get_transaction_by_hash(*transaction_hash)
    }

    pub fn get_latest_block(&self) -> DevnetResult<StarknetBlock> {
        let block = self
            .blocks
            .get_by_block_id(BlockId::Tag(starknet_rs_core::types::BlockTag::Latest))
            .ok_or(crate::error::Error::NoBlock)?;

        Ok(block.clone())
    }

    pub fn get_transaction_by_hash(&self, transaction_hash: Felt) -> DevnetResult<&Transaction> {
        self.transactions
            .get_by_hash(transaction_hash)
            .map(|starknet_transaction| &starknet_transaction.inner)
            .ok_or(Error::NoTransaction)
    }

    pub fn get_events(
        &self,
        from_block: Option<BlockId>,
        to_block: Option<BlockId>,
        address: Option<ContractAddress>,
        keys: Option<Vec<Vec<Felt>>>,
        skip: usize,
        limit: Option<usize>,
    ) -> DevnetResult<(Vec<EmittedEvent>, bool)> {
        events::get_events(self, from_block, to_block, address, keys, skip, limit)
    }

    pub fn get_transaction_receipt_by_hash(
        &self,
        transaction_hash: TransactionHash,
    ) -> DevnetResult<TransactionReceipt> {
        let transaction_to_map =
            self.transactions.get(&transaction_hash).ok_or(Error::NoTransaction)?;

        transaction_to_map.get_receipt()
    }

    pub fn simulate_transactions(
        &self,
        block_id: BlockId,
        transactions: &[BroadcastedTransaction],
        simulation_flags: Vec<SimulationFlag>,
    ) -> DevnetResult<Vec<SimulatedTransaction>> {
        let mut state = self.get_state_at(&block_id)?.clone();
        let chain_id = self.chain_id().to_felt();

        let mut skip_validate = false;
        let mut skip_fee_charge = false;
        for flag in simulation_flags.iter() {
            match flag {
                SimulationFlag::SkipValidate => {
                    skip_validate = true;
                    warn!("SKIP_VALIDATE chosen in simulation, but does not affect fee estimation");
                }
                SimulationFlag::SkipFeeCharge => skip_fee_charge = true,
            }
        }

        let mut transactions_traces: Vec<TransactionTrace> = vec![];

        for broadcasted_transaction in transactions.iter() {
            let blockifier_transaction =
                broadcasted_transaction.to_blockifier_account_transaction(chain_id)?;
            let tx_execution_info = blockifier_transaction.execute(
                &mut state.state,
                &self.block_context,
                !skip_fee_charge,
                !skip_validate,
            )?;
            let address_to_class_hash_map = &state.state.state.address_to_class_hash;

            let validate_invocation =
                if let Some(validate_info) = tx_execution_info.validate_call_info {
                    Some(FunctionInvocation::try_from_call_info(
                        validate_info,
                        address_to_class_hash_map,
                    )?)
                } else {
                    None
                };

            let fee_transfer_invocation =
                if let Some(fee_transfer_info) = tx_execution_info.fee_transfer_call_info {
                    Some(FunctionInvocation::try_from_call_info(
                        fee_transfer_info,
                        address_to_class_hash_map,
                    )?)
                } else {
                    None
                };

            let trace = match broadcasted_transaction {
                BroadcastedTransaction::Declare(_) => {
                    TransactionTrace::Declare(DeclareTransactionTrace {
                        validate_invocation,
                        fee_transfer_invocation,
                    })
                }
                BroadcastedTransaction::DeployAccount(_) => {
                    TransactionTrace::DeployAccount(DeployAccountTransactionTrace {
                        validate_invocation,
                        constructor_invocation: if let Some(call_info) =
                            tx_execution_info.execute_call_info
                        {
                            Some(FunctionInvocation::try_from_call_info(
                                call_info,
                                address_to_class_hash_map,
                            )?)
                        } else {
                            None
                        },
                        fee_transfer_invocation,
                    })
                }
                BroadcastedTransaction::Invoke(_) => {
                    TransactionTrace::Invoke(InvokeTransactionTrace {
                        validate_invocation,
                        execution_invocation: match tx_execution_info.execute_call_info {
                            Some(call_info) => match call_info.execution.failed {
                                false => ExecutionInvocation::Succeeded(
                                    FunctionInvocation::try_from_call_info(
                                        call_info,
                                        address_to_class_hash_map,
                                    )?,
                                ),
                                true => ExecutionInvocation::Reverted(
                                    starknet_types::rpc::transactions::Reversion {
                                        revert_reason: tx_execution_info
                                            .revert_error
                                            .unwrap_or("Revert reason not found".into()),
                                    },
                                ),
                            },
                            None => match tx_execution_info.revert_error {
                                Some(revert_reason) => ExecutionInvocation::Reverted(
                                    starknet_types::rpc::transactions::Reversion { revert_reason },
                                ),
                                None => {
                                    return Err(Error::UnexpectedInternalError {
                                        msg: "Simulation contains neither call_info nor \
                                              revert_error"
                                            .into(),
                                    });
                                }
                            },
                        },
                    })
                }
            };

            transactions_traces.push(trace);
        }

        let estimated = self.estimate_fee(block_id, transactions)?;

        // if the underlying simulation is correct, this should never be the case
        // in alignment with always avoiding assertions in production code, this has to be done
        if transactions_traces.len() != estimated.len() {
            return Err(Error::UnexpectedInternalError {
                msg: format!(
                    "Non-matching number of simulations ({}) and estimations ({})",
                    transactions_traces.len(),
                    estimated.len()
                ),
            });
        }

        let simulation_results = transactions_traces
            .into_iter()
            .zip(estimated)
            .map(|(trace, fee_estimation)| SimulatedTransaction {
                transaction_trace: trace,
                fee_estimation,
            })
            .collect();

        Ok(simulation_results)
    }
}

#[cfg(test)]
mod tests {
    use blockifier::state::state_api::State;
    use blockifier::transaction::errors::TransactionExecutionError;
    use starknet_api::block::{BlockHash, BlockNumber, BlockStatus, BlockTimestamp, GasPrice};
    use starknet_rs_core::types::{BlockId, BlockTag};
    use starknet_types::contract_address::ContractAddress;
    use starknet_types::felt::Felt;

    use super::Starknet;
    use crate::blocks::StarknetBlock;
    use crate::constants::{
        DEVNET_DEFAULT_CHAIN_ID, DEVNET_DEFAULT_INITIAL_BALANCE, ERC20_CONTRACT_ADDRESS,
    };
    use crate::error::{DevnetResult, Error};
    use crate::state::state_diff::StateDiff;
    use crate::traits::{Accounted, StateChanger, StateExtractor};
    use crate::utils::test_utils::{
        dummy_contract_address, dummy_declare_transaction_v1, dummy_felt, starknet_config_for_test,
    };

    #[test]
    fn correct_initial_state_with_test_config() {
        let config = starknet_config_for_test();
        let mut starknet = Starknet::new(&config).unwrap();
        let predeployed_accounts = starknet.predeployed_accounts.get_accounts();
        let expected_balance = config.predeployed_accounts_initial_balance;

        for account in predeployed_accounts {
            let account_balance = account.get_balance(&mut starknet.state).unwrap();
            assert_eq!(expected_balance, account_balance);
        }
    }

    #[test]
    fn correct_block_context_creation() {
        let fee_token_address =
            ContractAddress::new(Felt::from_prefixed_hex_str("0xAA").unwrap()).unwrap();
        let block_ctx = Starknet::init_block_context(10, "0xAA", DEVNET_DEFAULT_CHAIN_ID);
        assert_eq!(block_ctx.block_number, BlockNumber(0));
        assert_eq!(block_ctx.block_timestamp, BlockTimestamp(0));
        assert_eq!(block_ctx.gas_price, 10);
        assert_eq!(ContractAddress::from(block_ctx.fee_token_address), fee_token_address);
    }

    #[test]
    fn pending_block_is_correct() {
        let config = starknet_config_for_test();
        let mut starknet = Starknet::new(&config).unwrap();
        let initial_block_number = starknet.block_context.block_number;
        starknet.generate_pending_block().unwrap();

        assert_eq!(starknet.pending_block().header.block_number, initial_block_number.next());
    }

    #[test]
    fn correct_new_block_creation() {
        let config = starknet_config_for_test();
        let mut starknet = Starknet::new(&config).unwrap();

        let tx = dummy_declare_transaction_v1();

        // add transaction hash to pending block
        starknet.blocks.pending_block.add_transaction(tx.transaction_hash);

        // pending block has some transactions
        assert!(!starknet.pending_block().get_transactions().is_empty());
        // blocks collection is empty
        assert!(starknet.blocks.num_to_block.is_empty());

        starknet.generate_new_block(StateDiff::default()).unwrap();
        // blocks collection should not be empty
        assert!(!starknet.blocks.num_to_block.is_empty());

        // get block by number and check that the transactions in the block are correct
        let added_block = starknet.blocks.num_to_block.get(&BlockNumber(0)).unwrap();

        assert!(added_block.get_transactions().len() == 1);
        assert_eq!(*added_block.get_transactions().first().unwrap(), tx.transaction_hash);
    }

    #[test]
    fn successful_emptying_of_pending_block() {
        let config = starknet_config_for_test();
        let mut starknet = Starknet::new(&config).unwrap();

        let initial_block_number = starknet.block_context.block_number;
        let initial_gas_price = starknet.block_context.gas_price;
        let initial_block_timestamp = starknet.block_context.block_timestamp;
        let initial_sequencer = starknet.block_context.sequencer_address;

        // create pending block with some information in it
        let mut pending_block = StarknetBlock::create_pending_block();
        pending_block.add_transaction(dummy_felt());
        pending_block.status = BlockStatus::AcceptedOnL2;

        // assign the pending block
        starknet.blocks.pending_block = pending_block.clone();
        assert!(*starknet.pending_block() == pending_block);

        // empty the pending to block and check if it is in starting state
        starknet.restart_pending_block().unwrap();

        assert!(*starknet.pending_block() != pending_block);
        assert_eq!(starknet.pending_block().status, BlockStatus::Pending);
        assert!(starknet.pending_block().get_transactions().is_empty());
        assert_eq!(starknet.pending_block().header.timestamp, initial_block_timestamp);
        assert_eq!(starknet.pending_block().header.block_number, initial_block_number);
        assert_eq!(starknet.pending_block().header.parent_hash, BlockHash::default());
        assert_eq!(starknet.pending_block().header.gas_price, GasPrice(initial_gas_price));
        assert_eq!(starknet.pending_block().header.sequencer, initial_sequencer);
    }

    #[test]
    fn correct_block_context_update() {
        let mut block_ctx = Starknet::init_block_context(0, "0x0", DEVNET_DEFAULT_CHAIN_ID);
        let initial_block_number = block_ctx.block_number;
        Starknet::update_block_context(&mut block_ctx);

        assert_eq!(block_ctx.block_number, initial_block_number.next());
    }

    #[test]
    fn getting_state_of_latest_block() {
        let config = starknet_config_for_test();
        let starknet = Starknet::new(&config).unwrap();
        starknet.get_state_at(&BlockId::Tag(BlockTag::Latest)).expect("Should be OK");
    }

    #[test]
    fn getting_state_of_pending_block() {
        let config = starknet_config_for_test();
        let starknet = Starknet::new(&config).unwrap();
        starknet.get_state_at(&BlockId::Tag(BlockTag::Pending)).expect("Should be OK");
    }

    #[test]
    fn getting_state_at_block_by_nonexistent_hash() {
        let config = starknet_config_for_test();
        let mut starknet = Starknet::new(&config).unwrap();
        starknet.generate_new_block(StateDiff::default()).unwrap();

        match starknet.get_state_at(&BlockId::Hash(Felt::from(0).into())) {
            Err(Error::NoBlock) => (),
            _ => panic!("Should have failed"),
        }
    }

    #[test]
    fn getting_nonexistent_state_at_block_by_number() {
        let config = starknet_config_for_test();
        let mut starknet = Starknet::new(&config).unwrap();
        starknet.generate_new_block(StateDiff::default()).unwrap();
        starknet.blocks.num_to_state.remove(&BlockNumber(0));

        match starknet.get_state_at(&BlockId::Number(0)) {
            Err(Error::NoStateAtBlock { block_number: _ }) => (),
            _ => panic!("Should have failed"),
        }
    }

    #[test]
    fn calling_method_of_undeployed_contract() {
        let config = starknet_config_for_test();
        let starknet = Starknet::new(&config).unwrap();

        let undeployed_address_hex = "0x1234";
        let undeployed_address = Felt::from_prefixed_hex_str(undeployed_address_hex).unwrap();
        let entry_point_selector =
            starknet_rs_core::utils::get_selector_from_name("balanceOf").unwrap();

        match starknet.call(
            BlockId::Tag(BlockTag::Latest),
            undeployed_address,
            entry_point_selector.into(),
            vec![],
        ) {
            Err(Error::ContractNotFound) => (),
            unexpected => panic!("Should have failed; got {unexpected:?}"),
        }
    }

    #[test]
    fn calling_nonexistent_contract_method() {
        let config = starknet_config_for_test();
        let starknet = Starknet::new(&config).unwrap();

        let predeployed_account = &starknet.predeployed_accounts.get_accounts()[0];
        let entry_point_selector =
            starknet_rs_core::utils::get_selector_from_name("nonExistentMethod").unwrap();

        match starknet.call(
            BlockId::Tag(BlockTag::Latest),
            Felt::from_prefixed_hex_str(ERC20_CONTRACT_ADDRESS).unwrap(),
            entry_point_selector.into(),
            vec![Felt::from(predeployed_account.account_address)],
        ) {
            Err(Error::BlockifierTransactionError(
                TransactionExecutionError::EntryPointExecutionError(
                    blockifier::execution::errors::EntryPointExecutionError::PreExecutionError(
                        blockifier::execution::errors::PreExecutionError::EntryPointNotFound(_),
                    ),
                ),
            )) => (),
            unexpected => panic!("Should have failed; got {unexpected:?}"),
        }
    }

    /// utility method for happy path balance retrieval
    fn get_balance_at(
        starknet: &Starknet,
        contract_address: ContractAddress,
    ) -> DevnetResult<Vec<Felt>> {
        let entry_point_selector =
            starknet_rs_core::utils::get_selector_from_name("balanceOf").unwrap();
        starknet.call(
            BlockId::Tag(BlockTag::Latest),
            Felt::from_prefixed_hex_str(ERC20_CONTRACT_ADDRESS)?,
            entry_point_selector.into(),
            vec![Felt::from(contract_address)],
        )
    }

    #[test]
    fn getting_balance_of_predeployed_contract() {
        let config = starknet_config_for_test();
        let starknet = Starknet::new(&config).unwrap();

        let predeployed_account = &starknet.predeployed_accounts.get_accounts()[0];
        let result = get_balance_at(&starknet, predeployed_account.account_address).unwrap();

        let balance_hex = format!("0x{:x}", DEVNET_DEFAULT_INITIAL_BALANCE);
        let balance_felt = Felt::from_prefixed_hex_str(balance_hex.as_str()).unwrap();
        let balance_uint256 = vec![balance_felt, Felt::from_prefixed_hex_str("0x0").unwrap()];
        assert_eq!(result, balance_uint256);
    }

    #[test]
    fn getting_balance_of_undeployed_contract() {
        let config = starknet_config_for_test();
        let starknet = Starknet::new(&config).unwrap();

        let undeployed_address =
            ContractAddress::new(Felt::from_prefixed_hex_str("0x1234").unwrap()).unwrap();
        let result = get_balance_at(&starknet, undeployed_address).unwrap();

        let zero = Felt::from_prefixed_hex_str("0x0").unwrap();
        let expected_balance_uint256 = vec![zero, zero];
        assert_eq!(result, expected_balance_uint256);
    }

    #[test]
    fn correct_latest_block() {
        let config = starknet_config_for_test();
        let mut starknet = Starknet::new(&config).unwrap();

        starknet.get_latest_block().err().unwrap();

        starknet.generate_new_block(StateDiff::default()).unwrap();
        starknet.generate_pending_block().unwrap();

        // last added block number -> 0
        let added_block = starknet.blocks.num_to_block.get(&BlockNumber(0)).unwrap();
        // number of the accepted block -> 1
        let block_number = starknet.get_latest_block().unwrap().block_number();

        assert_eq!(block_number.0, added_block.header.block_number.0);

        starknet.generate_new_block(StateDiff::default()).unwrap();
        starknet.generate_pending_block().unwrap();

        let added_block2 = starknet.blocks.num_to_block.get(&BlockNumber(1)).unwrap();
        let block_number2 = starknet.get_latest_block().unwrap().block_number();

        assert_eq!(block_number2.0, added_block2.header.block_number.0);
    }

    #[test]
    fn gets_block_txs_count() {
        let config = starknet_config_for_test();
        let mut starknet = Starknet::new(&config).unwrap();

        starknet.generate_new_block(StateDiff::default()).unwrap();
        starknet.generate_pending_block().unwrap();

        let num_no_transactions = starknet.get_block_txs_count(BlockId::Number(0));

        assert_eq!(num_no_transactions.unwrap(), 0);

        let tx = dummy_declare_transaction_v1();

        // add transaction hash to pending block
        starknet.blocks.pending_block.add_transaction(tx.transaction_hash);

        starknet.generate_new_block(StateDiff::default()).unwrap();

        let num_one_transaction = starknet.get_block_txs_count(BlockId::Number(1));

        assert_eq!(num_one_transaction.unwrap(), 1);
    }

    #[test]
    fn returns_chain_id() {
        let config = starknet_config_for_test();
        let starknet = Starknet::new(&config).unwrap();
        let chain_id = starknet.chain_id();

        assert_eq!(chain_id.to_string(), DEVNET_DEFAULT_CHAIN_ID.to_string());
    }

    #[test]
    fn correct_state_at_specific_block() {
        let mut starknet = Starknet::default();
        // generate initial block with empty state
        starknet.generate_new_block(StateDiff::default()).unwrap();
        starknet.generate_pending_block().unwrap();

        // **generate second block**
        // add data to state
        starknet.state.state.increment_nonce(dummy_contract_address().try_into().unwrap()).unwrap();
        // get state difference
        let state_diff = starknet.state.extract_state_diff_from_pending_state().unwrap();
        // move data from pending_state to state
        starknet.state.apply_state_difference(state_diff.clone()).unwrap();
        // generate new block and save the state
        let second_block = starknet.generate_new_block(state_diff).unwrap();
        starknet.generate_pending_block().unwrap();

        // **generate third block**
        // add data to state
        starknet.state.state.increment_nonce(dummy_contract_address().try_into().unwrap()).unwrap();
        // get state difference
        let state_diff = starknet.state.extract_state_diff_from_pending_state().unwrap();
        // move data from pending_state to state
        starknet.state.apply_state_difference(state_diff.clone()).unwrap();
        // generate new block and save the state
        let third_block = starknet.generate_new_block(state_diff).unwrap();
        starknet.generate_pending_block().unwrap();

        // check modified state at block 1 and 2 to contain the correct value for the nonce
        let second_block_address_nonce = starknet
            .blocks
            .num_to_state
            .get(&second_block)
            .unwrap()
            .state
            .state
            .address_to_nonce
            .get(&dummy_contract_address())
            .unwrap();
        let second_block_expected_address_nonce = Felt::from(1);
        assert_eq!(second_block_expected_address_nonce, *second_block_address_nonce);

        let third_block_address_nonce = starknet
            .blocks
            .num_to_state
            .get(&third_block)
            .unwrap()
            .state
            .state
            .address_to_nonce
            .get(&dummy_contract_address())
            .unwrap();
        let third_block_expected_address_nonce = Felt::from(2);
        assert_eq!(third_block_expected_address_nonce, *third_block_address_nonce);
    }

    #[test]
    fn gets_latest_block() {
        let config = starknet_config_for_test();
        let mut starknet = Starknet::new(&config).unwrap();

        starknet.generate_new_block(StateDiff::default()).unwrap();
        starknet.generate_pending_block().unwrap();
        starknet.generate_new_block(StateDiff::default()).unwrap();
        starknet.generate_pending_block().unwrap();
        starknet.generate_new_block(StateDiff::default()).unwrap();

        let latest_block = starknet.get_latest_block();

        assert_eq!(latest_block.unwrap().block_number(), BlockNumber(2));
    }
}
