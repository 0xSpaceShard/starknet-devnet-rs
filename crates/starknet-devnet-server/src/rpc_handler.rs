use std::fmt::{self};

use axum::extract::rejection::JsonRejection;
use axum::extract::State;
use axum::Json;
use futures::{future, FutureExt};
use serde::de::DeserializeOwned;
use tracing::{error, trace, warn};

use crate::rpc_core::error::RpcError;
use crate::rpc_core::request::{Request, RpcCall, RpcMethodCall};
use crate::rpc_core::response::{Response, ResponseResult, RpcResponse};

/// Helper trait that is used to execute starknet rpc calls
#[async_trait::async_trait]
pub trait RpcHandler: Clone + Send + Sync + 'static {
    /// The request type to expect
    type Request: DeserializeOwned + Send + Sync + fmt::Display;

    /// Invoked when the request was received
    async fn on_request(
        &self,
        request: Self::Request,
        original_call: RpcMethodCall,
    ) -> ResponseResult;

    /// Invoked for every incoming `RpcMethodCall`
    ///
    /// This will attempt to deserialize a `{ "method" : "<name>", "params": "<params>" }` message
    /// into the `Request` type of this handler. If a `Request` instance was deserialized
    /// successfully, [`Self::on_request`] will be invoked.
    ///
    /// **Note**: override this function if the expected `Request` deviates from `{ "method" :
    /// "<name>", "params": "<params>" }`
    /// TODO can call be a reference? could prevent cloning in other parts of the code
    async fn on_call(&self, call: RpcMethodCall) -> RpcResponse {
        trace!(target: "rpc",  id = ?call.id , method = ?call.method, "received method call");
        let RpcMethodCall { method, params, id, .. } = call.clone();

        let params: serde_json::Value = params.into();
        let deserializable_call = serde_json::json!({
            "method": &method,
            "params": params
        });

        match serde_json::from_value::<Self::Request>(deserializable_call) {
            Ok(req) => {
                let result = self.on_request(req, call).await;
                RpcResponse::new(id, result)
            }
            Err(err) => {
                let err = err.to_string();
                // since JSON-RPC specification requires returning a Method Not Found error,
                // we apply a hacky way to induce this - checking the stringified error message
                let distinctive_error = format!("unknown variant `{method}`");
                if err.contains(&distinctive_error) {
                    error!(target: "rpc", ?method, "failed to deserialize method due to unknown variant");
                    RpcResponse::new(id, RpcError::method_not_found())
                } else {
                    error!(target: "rpc", ?method, ?err, "failed to deserialize method");
                    RpcResponse::new(id, RpcError::invalid_params(err))
                }
            }
        }
    }
}

/// Handles incoming JSON-RPC Request
pub async fn handle<THandler: RpcHandler>(
    State(handler): State<THandler>,
    request: Result<Json<Request>, JsonRejection>,
) -> Json<Response> {
    match request {
        Ok(req) => handle_request(req.0, handler)
            .await
            .unwrap_or_else(|| Response::error(RpcError::invalid_request()))
            .into(),
        Err(err) => {
            warn!(target: "rpc", ?err, "invalid request");
            Response::error(RpcError::invalid_request()).into()
        }
    }
}

#[macro_export]
macro_rules! http_rpc_router {
    // Match a list of pairs enclosed in parentheses
    ( $( ( $http_path:expr, $rpc_method_name:ident ) ),* $(,)?  ) => {
        {
            use axum::extract::State;
            use axum::Json;
            use $crate::rpc_core::error::RpcError;
            use $crate::rpc_core::request::Version;
            use $crate::rpc_core::request::Id;
            use $crate::rpc_core::request::RpcCall;
            use $crate::rpc_core::request::RpcMethodCall;
            use $crate::rpc_core::request::RequestParams;
            use $crate::rpc_core::request::Request;
            use $crate::rpc_core::response::Response;
            use $crate::rpc_handler::handle_request;

            let mut router = Router::new();
            $(
                #[allow(non_snake_case)]
                pub async fn $rpc_method_name<THandler: RpcHandler>(
                    State(handler): State<THandler>,
                    Json(request): Json<serde_json::Map<String, serde_json::Value>>,
                ) -> Json<serde_json::Value>{
                    let rpc_req = Json(Request::Single(RpcCall::MethodCall(RpcMethodCall {
                        jsonrpc: Version::V2,
                        method: stringify!($rpc_method_name).to_string(),
                        params: RequestParams::Object(request),
                        id: Id::Number(0),
                    })));
                    let rpc_resp: Response = handle_request(rpc_req.0, handler)
                        .await
                        .unwrap_or_else(|| Response::error(RpcError::invalid_request()))
                        .into();

                    let rpc_resp = match serde_json::to_value(rpc_resp) {
                        Ok(r) => r,
                        Err(e) => return Json(serde_json::json!({ "error": e.to_string() }))
                    };

                    let non_rpc_resp: serde_json::Value = if let Some(result) = rpc_resp.get("result") {
                        result.clone()
                    } else if let Some(error) = rpc_resp.get("error") {
                        error.clone()
                    } else {
                        rpc_resp
                    };
                    Json(non_rpc_resp)
                }

                router = router.route($http_path, post($rpc_method_name::<JsonRpcHandler>));
            )*
            router
        }
    };
}

/// Handle the JSON-RPC [Request]
///
/// This will try to deserialize the payload into the request type of the handler and if successful
/// invoke the handler.
pub async fn handle_request<THandler: RpcHandler>(
    req: Request,
    handler: THandler,
) -> Option<Response> {
    /// processes batch calls
    fn responses_as_batch(outs: Vec<Option<RpcResponse>>) -> Option<Response> {
        let batch: Vec<_> = outs.into_iter().flatten().collect();
        (!batch.is_empty()).then_some(Response::Batch(batch))
    }

    match req {
        Request::Single(call) => handle_call(call, handler).await.map(Response::Single),
        Request::Batch(calls) => {
            future::join_all(calls.into_iter().map(move |call| handle_call(call, handler.clone())))
                .map(responses_as_batch)
                .await
        }
    }
}

/// handle a single RPC method call
async fn handle_call<THandler: RpcHandler>(
    call: RpcCall,
    handler: THandler,
) -> Option<RpcResponse> {
    match call {
        RpcCall::MethodCall(call) => {
            trace!(target: "rpc", id = ?call.id , method = ?call.method,  "handling call");
            Some(handler.on_call(call).await)
        }
        RpcCall::Notification(notification) => {
            trace!(target: "rpc", method = ?notification.method, "received rpc notification");
            None
        }
        RpcCall::Invalid { id } => {
            warn!(target: "rpc", ?id,  "invalid rpc call");
            Some(RpcResponse::invalid_request(id))
        }
    }
}
