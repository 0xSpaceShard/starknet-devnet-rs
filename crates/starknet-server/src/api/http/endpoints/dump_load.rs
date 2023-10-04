use axum::{Extension, Json};

use crate::api::http::error::HttpApiError;
use crate::api::http::models::{DumpLoadResponse, Path};
use crate::api::http::{HttpApiHandler, HttpApiResult};

pub(crate) async fn dump(
    Json(path): Json<Path>,
    Extension(state): Extension<HttpApiHandler>,
) -> HttpApiResult<Json<DumpLoadResponse>> {
    let starknet = state.api.starknet.write().await;
    starknet
        .dump_transactions_custom_path(Some(path.path.clone()))
        .map_err(|_| HttpApiError::GeneralError)?;

    Ok(Json(DumpLoadResponse { path: path.path }))
}

pub(crate) async fn load(
    Json(path): Json<Path>,
    Extension(state): Extension<HttpApiHandler>,
) -> HttpApiResult<Json<DumpLoadResponse>> {
    let mut starknet = state.api.starknet.write().await;
    let transactions = starknet
        .load_transactions_custom_path(Some(path.path.clone()))
        .map_err(|_| HttpApiError::GeneralError)?;
    println!("transactions: {:?}", transactions);
    let result = starknet.re_execute(transactions).map_err(|_| HttpApiError::GeneralError)?;
    println!("result: {:?}", result);

    Ok(Json(DumpLoadResponse { path: path.path }))
}
