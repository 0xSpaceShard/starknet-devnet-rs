use axum::extract::State;
use axum::Json;

use crate::api::http::error::HttpApiError;
use crate::api::http::models::{AbortedBlocks, AbortingBlocks, CreatedBlock, GasUpdate};
use crate::api::http::{HttpApiHandler, HttpApiResult};
use crate::api::Api;

pub async fn create_block(
    State(state): State<HttpApiHandler>,
) -> HttpApiResult<Json<CreatedBlock>> {
    create_block_impl(&state.api).await.map(Json::from)
}

pub(crate) async fn create_block_impl(api: &Api) -> HttpApiResult<CreatedBlock> {
    let mut starknet = api.starknet.write().await;
    starknet
        .create_block_dump_event(None)
        .map_err(|err| HttpApiError::CreateEmptyBlockError { msg: err.to_string() })?;

    let last_block = starknet.get_latest_block();
    match last_block {
        Ok(block) => Ok(CreatedBlock { block_hash: block.block_hash() }),
        Err(err) => Err(HttpApiError::CreateEmptyBlockError { msg: err.to_string() }),
    }
}

pub async fn abort_blocks(
    State(state): State<HttpApiHandler>,
    Json(data): Json<AbortingBlocks>,
) -> HttpApiResult<Json<AbortedBlocks>> {
    abort_blocks_impl(&state.api, data).await.map(Json::from)
}

pub(crate) async fn abort_blocks_impl(
    api: &Api,
    data: AbortingBlocks,
) -> HttpApiResult<AbortedBlocks> {
    let mut starknet = api.starknet.write().await;
    let aborted = starknet
        .abort_blocks(data.starting_block_hash)
        .map_err(|err| HttpApiError::BlockAbortError { msg: (err.to_string()) })?;

    Ok(AbortedBlocks { aborted })
}

pub async fn update_gas(
    State(state): State<HttpApiHandler>,
    Json(data): Json<GasUpdate>,
) -> HttpApiResult<Json<GasUpdate>> {
    update_gas_impl(&state.api, data).await.map(Json::from)
}

pub(crate) async fn update_gas_impl(api: &Api, data: GasUpdate) -> HttpApiResult<GasUpdate> {
    let mut starknet = api.starknet.write().await;
    let updated = starknet
        .update_gas(
            data.gas_price_wei,
            data.data_gas_price_wei,
            data.gas_price_strk,
            data.data_gas_price_strk,
        )
        .map_err(|err| HttpApiError::BlockAbortError { msg: (err.to_string()) })?;

    // TODO: change to struct not tuples
    Ok(GasUpdate {
        gas_price_wei: updated.0,
        data_gas_price_wei: updated.1,
        gas_price_strk: updated.2,
        data_gas_price_strk: updated.3,
    })
}
