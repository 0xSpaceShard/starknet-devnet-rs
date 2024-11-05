use starknet_core::error::Error;
use starknet_rs_core::types::{BlockId, BlockTag};

use super::error::ApiError;
use super::models::{BlockIdInput, SubscriptionIdInput};
use super::{JsonRpcHandler, JsonRpcSubscriptionRequest};
use crate::rpc_core::request::Id;
use crate::subscribe::{SocketId, SubscriptionNotification};

/// The definitions of JSON-RPC read endpoints defined in starknet_ws_api.json
impl JsonRpcHandler {
    pub async fn execute_ws(
        &self,
        request: JsonRpcSubscriptionRequest,
        rpc_request_id: Id,
        socket_id: SocketId,
    ) -> Result<(), ApiError> {
        match request {
            JsonRpcSubscriptionRequest::NewHeads(data) => {
                self.subscribe_new_heads(data, rpc_request_id, socket_id).await
            }
            JsonRpcSubscriptionRequest::TransactionStatus => todo!(),
            JsonRpcSubscriptionRequest::PendingTransactions => todo!(),
            JsonRpcSubscriptionRequest::Events => todo!(),
            JsonRpcSubscriptionRequest::Unsubscribe(SubscriptionIdInput { subscription_id }) => {
                let mut sockets = self.api.sockets.lock().await;
                let socket_context = sockets.get_mut(&socket_id).ok_or(
                    ApiError::StarknetDevnetError(Error::UnexpectedInternalError {
                        msg: format!("Unregistered socket ID: {socket_id}"),
                    }),
                )?;

                socket_context.unsubscribe(rpc_request_id, subscription_id).await?;
                Ok(())
            }
        }
    }

    /// starknet_subscribeNewHeads
    pub async fn subscribe_new_heads(
        &self,
        block_id_input: Option<BlockIdInput>,
        rpc_request_id: Id,
        socket_id: SocketId,
    ) -> Result<(), ApiError> {
        let latest_tag = BlockId::Tag(BlockTag::Latest);
        let block_id = if let Some(BlockIdInput { block_id }) = block_id_input {
            block_id.into()
        } else {
            latest_tag
        };

        let starknet = self.api.starknet.lock().await;

        // checking the block's existence; TODO what if block aborted
        let query_block = starknet.get_block(&block_id).map_err(|e| match e {
            Error::NoBlock => ApiError::BlockNotFound,
            unknown_error => ApiError::StarknetDevnetError(unknown_error),
        })?;

        let latest_block = starknet.get_block(&latest_tag)?;

        let query_block_number = query_block.block_number().0;
        let latest_block_number = latest_block.block_number().0;

        let blocks_back_amount = if query_block_number > latest_block_number {
            0
        } else {
            latest_block_number - query_block_number
        };

        if blocks_back_amount > 1024 {
            return Err(ApiError::TooManyBlocksBack);
        }

        let mut sockets = self.api.sockets.lock().await;
        let socket_context = sockets.get_mut(&socket_id).ok_or(ApiError::StarknetDevnetError(
            Error::UnexpectedInternalError { msg: format!("Unregistered socket ID: {socket_id}") },
        ))?;

        let subscription_id = socket_context.subscribe(rpc_request_id).await;

        if let BlockId::Tag(_) = block_id {
            // if the specified block ID is a tag (i.e. latest/pending), no old block handling
            return Ok(());
        }

        // latest_block_number inclusive? Yes, only if block_id != tag (already taken care of)
        for block_n in query_block_number..=latest_block_number {
            let old_block = starknet
                .get_block(&BlockId::Number(block_n))
                .map_err(ApiError::StarknetDevnetError)?;

            let notification = SubscriptionNotification::NewHeadsNotification(old_block.into());
            socket_context.notify(subscription_id, notification).await;
        }

        Ok(())
    }
}