use axum::extract::State;
use axum::Json;
use starknet_types::rpc::messaging::{MessageToL1, MessageToL2};
use starknet_types::rpc::transactions::l1_handler_transaction::L1HandlerTransaction;

use super::extract_optional_json_from_request;
use crate::api::http::error::HttpApiError;
use crate::api::http::models::{
    FlushParameters, FlushedMessages, MessageHash, MessagingLoadAddress,
    PostmanLoadL1MessagingContract, TxHash,
};
use crate::api::http::{HttpApiHandler, HttpApiResult};
use crate::api::Api;

pub async fn postman_load(
    State(state): State<HttpApiHandler>,
    Json(data): Json<PostmanLoadL1MessagingContract>,
) -> HttpApiResult<Json<MessagingLoadAddress>> {
    postman_load_impl(&state.api, data).await.map(Json::from)
}

pub async fn postman_flush(
    State(state): State<HttpApiHandler>,
    optional_data: Option<Json<FlushParameters>>,
) -> HttpApiResult<Json<FlushedMessages>> {
    postman_flush_impl(&state.api, extract_optional_json_from_request(optional_data))
        .await
        .map(Json::from)
}

pub async fn postman_send_message_to_l2(
    State(state): State<HttpApiHandler>,
    Json(message): Json<MessageToL2>,
) -> HttpApiResult<Json<TxHash>> {
    postman_send_message_to_l2_impl(&state.api, message).await.map(Json::from)
}

pub async fn postman_consume_message_from_l2(
    State(state): State<HttpApiHandler>,
    Json(message): Json<MessageToL1>,
) -> HttpApiResult<Json<MessageHash>> {
    postman_consume_message_from_l2_impl(&state.api, message).await.map(Json::from)
}

pub(crate) async fn postman_load_impl(
    api: &Api,
    data: PostmanLoadL1MessagingContract,
) -> HttpApiResult<MessagingLoadAddress> {
    let mut starknet = api.starknet.lock().await;

    let messaging_contract_address = starknet
        .configure_messaging(&data.network_url, data.address.as_deref())
        .await
        .map_err(|e| HttpApiError::MessagingError { msg: e.to_string() })?;

    Ok(MessagingLoadAddress { messaging_contract_address })
}

pub(crate) async fn postman_flush_impl(
    api: &Api,
    data: Option<FlushParameters>,
) -> HttpApiResult<FlushedMessages> {
    // Need to handle L1 to L2 first in case that those messages
    // will create L2 to L1 messages.
    let mut starknet = api.starknet.lock().await;

    let is_dry_run = if let Some(params) = data { params.dry_run } else { false };

    // Fetch and execute messages to l2.
    let (messages_to_l2, generated_l2_transactions) = if is_dry_run {
        (vec![], vec![])
    } else {
        let messages = starknet.fetch_messages_to_l2().await.map_err(|e| {
            HttpApiError::MessagingError { msg: format!("fetch messages to l2: {}", e) }
        })?;

        let tx_hashes = starknet.execute_messages_to_l2(&messages).await.map_err(|e| {
            HttpApiError::MessagingError { msg: format!("execute messages to l2: {}", e) }
        })?;

        (messages, tx_hashes)
    };

    // Collect and send messages to L1.
    let messages_to_l1 = starknet.collect_messages_to_l1().await.map_err(|e| {
        HttpApiError::MessagingError { msg: format!("collect messages to l1 error: {}", e) }
    })?;

    if is_dry_run {
        return Ok(FlushedMessages {
            messages_to_l1,
            messages_to_l2,
            generated_l2_transactions,
            l1_provider: "dry run".to_string(),
        });
    }

    starknet.send_messages_to_l1().await.map_err(|e| HttpApiError::MessagingError {
        msg: format!("send messages to l1 error: {}", e),
    })?;

    let l1_provider = starknet.get_ethereum_url().unwrap_or("Not set".to_string());

    Ok(FlushedMessages { messages_to_l1, messages_to_l2, generated_l2_transactions, l1_provider })
}

pub async fn postman_send_message_to_l2_impl(
    api: &Api,
    message: MessageToL2,
) -> HttpApiResult<TxHash> {
    let mut starknet = api.starknet.lock().await;

    let transaction = L1HandlerTransaction::try_from_message_to_l2(message).map_err(|_| {
        HttpApiError::InvalidValueError {
            msg: "The `paid_fee_on_l1` is out of range, expecting u128 value".to_string(),
        }
    })?;

    let transaction_hash = starknet
        .add_l1_handler_transaction(transaction)
        .map_err(|e| HttpApiError::MessagingError { msg: e.to_string() })?;

    Ok(TxHash { transaction_hash })
}

pub async fn postman_consume_message_from_l2_impl(
    api: &Api,
    message: MessageToL1,
) -> HttpApiResult<MessageHash> {
    let mut starknet = api.starknet.lock().await;

    let message_hash = starknet
        .consume_l2_to_l1_message(&message)
        .await
        .map_err(|e| HttpApiError::MessagingError { msg: e.to_string() })?;

    Ok(MessageHash { message_hash })
}
