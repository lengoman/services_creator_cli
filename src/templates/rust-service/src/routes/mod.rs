use axum::{
    routing::get,
    Router,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    middleware::Next,
    extract::State,
};
use std::sync::Arc;

use crate::common::validation::{RapidApiConfig, validate_rapidapi_headers};

pub fn create_router(rapidapi_config: Arc<RapidApiConfig>) -> Router {
    Router::new()
        .route("/api/v1/hello", get(hello_handler))
        .with_state(rapidapi_config.clone())
        .layer(axum::middleware::from_fn_with_state(
            rapidapi_config,
            validate_rapidapi_middleware,
        ))
}

async fn validate_rapidapi_middleware(
    State(rapidapi_config): State<Arc<RapidApiConfig>>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // Validate RapidAPI headers
    validate_rapidapi_headers(request.headers(), &rapidapi_config)
        .await
        .map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                format!("RapidAPI validation failed: {:?}", e),
            )
        })?;

    // If validation passes, continue to the next middleware/handler
    Ok(next.run(request).await)
}

async fn hello_handler() -> String {
    "Hello, RapidAPI!".to_string()
}
