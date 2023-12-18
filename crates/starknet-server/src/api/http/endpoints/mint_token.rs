use axum::{Extension, Json};
use starknet_core::constants::{ETH_ERC20_CONTRACT_ADDRESS, STRK_ERC20_CONTRACT_ADDRESS};
use starknet_core::starknet::Starknet;
use starknet_rs_core::types::{BlockId, BlockTag};
use starknet_types::contract_address::ContractAddress;
use starknet_types::felt::Felt;
use starknet_types::num_bigint::BigUint;

use crate::api::http::error::HttpApiError;
use crate::api::http::models::{FeeToken, MintTokensRequest, MintTokensResponse};
use crate::api::http::{HttpApiHandler, HttpApiResult};
use crate::api::json_rpc::error::ApiError;

pub(crate) async fn get_fee_token() -> HttpApiResult<Json<FeeToken>> {
    Err(HttpApiError::GeneralError)
}

/// get the balance of the `address`
fn get_balance(starknet: &Starknet, address: ContractAddress, erc20_contract: &str) -> Result<BigUint, ApiError> {
    let erc20_address = Felt::from_prefixed_hex_str(erc20_contract).unwrap();
    let balance_selector =
        starknet_rs_core::utils::get_selector_from_name("balanceOf").unwrap().into();
    let new_balance_raw = starknet.call(
        BlockId::Tag(BlockTag::Pending),
        erc20_address,
        balance_selector,
        vec![Felt::from(address)], // calldata = the address being queried
    )?;

    // format balance for output - initially it is a 2-member vector (low, high)
    if new_balance_raw.len() != 2 {
        let msg =
            format!("Fee token contract expected to return 2 values; got: {:?}", new_balance_raw);

        return Err(ApiError::ContractError {
            error: starknet_core::error::Error::UnexpectedInternalError { msg },
        });
    }
    let new_balance_low: BigUint = (*new_balance_raw.get(0).unwrap()).into();
    let new_balance_high: BigUint = (*new_balance_raw.get(1).unwrap()).into();
    let new_balance: BigUint = (new_balance_high << 128) + new_balance_low;
    Ok(new_balance)
}

pub(crate) async fn mint(
    Json(request): Json<MintTokensRequest>,
    Extension(state): Extension<HttpApiHandler>,
) -> HttpApiResult<Json<MintTokensResponse>> {
    let mut starknet = state.api.starknet.write().await;
    let mut unit = "WEI".to_string();
    let mut erc20_contract = ETH_ERC20_CONTRACT_ADDRESS;

    // if unit is FRI, change contract address and unit
    match request.unit {
        Some(u) => {
            if u == "FRI"
            {
                erc20_contract = STRK_ERC20_CONTRACT_ADDRESS;
                unit = u
            }
        },
        None => {},
    }

    // increase balance
    let tx_hash = starknet
        .mint(request.address, request.amount, erc20_contract)
        .await
        .map_err(|err| HttpApiError::MintingError { msg: err.to_string() })?;

    let new_balance = get_balance(&starknet, request.address, erc20_contract)
        .map_err(|err| HttpApiError::MintingError { msg: err.to_string() })?;

    Ok(Json(MintTokensResponse {
        new_balance: new_balance.to_str_radix(10),
        unit: unit,
        tx_hash: tx_hash,
    }))
}
