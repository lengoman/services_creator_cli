use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::common::validation::RapidApiConfig;

pub fn create_router(rapidapi_config: Arc<RapidApiConfig>) -> Router {
    Router::new()
        .route("/api/v1/hello", get(hello_handler))
        .with_state(rapidapi_config)
}

async fn hello_handler() -> String {
    "helloworld".to_string()
}

