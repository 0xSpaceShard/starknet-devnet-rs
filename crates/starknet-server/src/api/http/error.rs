use axum::response::IntoResponse;
use axum::Json;
use hyper::StatusCode;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpApiError {
    #[error("General error")]
    GeneralError,
    #[error("Minting error: {msg}")]
    MintingError { msg: String },
    #[error("The file does not exist")]
    FileNotFound,
    #[error("The dump operation failed")]
    DumpError { msg: String },
    #[error("The load operation failed")]
    LoadError,
    #[error("The re-execution operation failed")]
    ReExecutionError,
    #[error("The creation of empty block failed")]
    CreateEmptyBlockError,
    #[error("The set time operation failed")]
    BlockSetTimeError,
    #[error("The increase time operation failed")]
    BlockIncreaseTimeError,
}

impl IntoResponse for HttpApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            HttpApiError::GeneralError => {
                (StatusCode::INTERNAL_SERVER_ERROR, HttpApiError::GeneralError.to_string())
            }
            HttpApiError::FileNotFound => {
                (StatusCode::BAD_REQUEST, HttpApiError::FileNotFound.to_string())
            }
            err @ HttpApiError::DumpError { msg: _ } => (StatusCode::BAD_REQUEST, err.to_string()),
            HttpApiError::LoadError => {
                (StatusCode::BAD_REQUEST, HttpApiError::LoadError.to_string())
            }
            HttpApiError::ReExecutionError => {
                (StatusCode::BAD_REQUEST, HttpApiError::ReExecutionError.to_string())
            }
            HttpApiError::CreateEmptyBlockError => {
                (StatusCode::BAD_REQUEST, HttpApiError::CreateEmptyBlockError.to_string())
            }
            HttpApiError::BlockSetTimeError => {
                (StatusCode::BAD_REQUEST, HttpApiError::BlockSetTimeError.to_string())
            }
            HttpApiError::BlockIncreaseTimeError => {
                (StatusCode::BAD_REQUEST, HttpApiError::BlockIncreaseTimeError.to_string())
            }
            err @ HttpApiError::MintingError { msg: _ } => {
                (StatusCode::BAD_REQUEST, err.to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
