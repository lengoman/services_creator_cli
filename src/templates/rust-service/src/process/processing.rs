
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};

use axum::body::{Body, Bytes};
use axum::extract::{FromRequest, Request};
use axum::response::Response;
use serde::de::DeserializeOwned;
use serde::Serialize;


#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String
}

#[derive(Debug)]
pub struct CustomJson<T>(pub T);

#[axum::async_trait]
impl<T, S> FromRequest<S, Body> for CustomJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let bytes = Bytes::from_request(req, state)
            .await
            .map_err(|e| {
                let error = ErrorResponse {
                    error: format!("Failed to read request body: {}", e)
                };
                (StatusCode::OK, Json(error)).into_response()
            })?;

        let value: T = serde_json::from_slice(&bytes).map_err(|e| {
            let error = ErrorResponse {
                error: format!("Failed to deserialize request body: {}", e)
            };
            (StatusCode::OK, Json(error)).into_response()
        })?;

        Ok(CustomJson(value))
    }
}
