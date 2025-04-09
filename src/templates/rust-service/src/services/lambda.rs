use lambda_http::{run, service_fn, Body, Error, Request, Response, tower::ServiceExt};
use std::sync::Arc;
use tracing::info;

use {{crate_name}}::common::validation::RapidApiConfig;
use {{crate_name}}::routes;

async fn function_handler(
    event: Request,
    rapidapi_config: Arc<RapidApiConfig>,
) -> Result<Response<Body>, Error> {
    // Create the router
    let app = routes::create_router(rapidapi_config);
    
    // Process the request using the router
    let response = app.oneshot(event).await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    // Load RapidAPI configuration from environment
    let rapidapi_config = Arc::new(RapidApiConfig::from_env().map_err(|e| {
        Error::from(format!("Failed to load RapidAPI config: {}", e))
    })?);

    info!("Starting Lambda handler");
    
    run(service_fn(|event| {
        function_handler(event, rapidapi_config.clone())
    }))
    .await
}
