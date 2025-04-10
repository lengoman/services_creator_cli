use axum::body::Body;
use lambda_http::{run, service_fn, Error, Request, Response};
use tower::ServiceExt;
use uuid::Uuid;
use http_body_util::BodyExt;
use tracing::{Level, info};
use tracing_subscriber;
use std::sync::Arc;

use {{project-name}}::common::validation::RapidApiConfig;
use {{project-name}}::routes::create_router;

// Convert lambda_http::Request to axum::http::Request
#[allow(dead_code)]
fn lambda_to_axum_request(lambda_request: Request) -> axum::http::Request<Body> {
    let (parts, body) = lambda_request.into_parts();
    let body = match body {
        lambda_http::Body::Empty => Body::empty(),
        lambda_http::Body::Text(text) => Body::from(text),
        lambda_http::Body::Binary(data) => Body::from(data),
    };

    // Construct the full URI with query parameters
    let uri = if let Some(query) = parts.uri.query() {
        format!("{}?{}", parts.uri.path(), query)
    } else {
        parts.uri.path().to_string()
    };

    let mut builder = axum::http::Request::builder()
        .method(parts.method)
        .uri(uri);

    // Add headers
    for (key, value) in parts.headers.iter() {
        builder = builder.header(key, value);
    }

    // Build the request
    builder.body(body).unwrap_or_else(|_| {
        eprintln!("Failed to build axum request");
        axum::http::Request::new(Body::empty())
    })
}

// Convert axum::http::Response to lambda_http::Response
#[allow(dead_code)]
async fn axum_to_lambda_response(axum_response: axum::http::Response<Body>) -> Response<lambda_http::Body> {
    let (parts, body) = axum_response.into_parts();

    // Convert the body to bytes
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => {
            return Response::builder()
                .status(500)
                .body(lambda_http::Body::Text("Failed to read response body".to_string()))
                .unwrap();
        }
    };

    // Convert to appropriate body type
    let lambda_body = if bytes.is_empty() {
        lambda_http::Body::Empty
    } else {
        // Try to convert to text first
        match String::from_utf8(bytes.to_vec()) {
            Ok(text) => lambda_http::Body::Text(text),
            Err(_) => lambda_http::Body::Binary(bytes.to_vec()),
        }
    };

    // Build the response
    let mut builder = Response::builder()
        .status(parts.status);

    // Add headers
    for (key, value) in parts.headers.iter() {
        builder = builder.header(key, value);
    }

    builder.body(lambda_body).unwrap_or_else(|_| {
        eprintln!("Failed to build lambda response");
        Response::builder()
            .status(500)
            .body(lambda_http::Body::Text("Internal Server Error".to_string()))
            .unwrap()
    })
}

// Lambda handler function
#[allow(dead_code)]
async fn handler(lambda_request: Request) -> Result<Response<lambda_http::Body>, Error> {
    // Generate a unique server ID for this Lambda instance
    let server_id = Uuid::new_v4().to_string();
    info!("Lambda instance started with server_id: {}", server_id);
    info!("Received request: {} {}", lambda_request.method(), lambda_request.uri());
    if let Some(query) = lambda_request.uri().query() {
        info!("Query parameters: {}", query);
    }

    // Configure RapidAPI settings
    let rapidapi_config = Arc::new(RapidApiConfig::new(
        "6f79849e04msh105d56a90bc7568p11ccd6jsn0c8c9c6fde05",
        "059fe2c0-adcd-11ef-9c3f-a75c3ffbbfe4",
        "qr-code-generator-logo.p.rapidapi.com"
    ));

    // Create the router
    let app = create_router(rapidapi_config);

    // Convert lambda request to axum request
    let axum_request = lambda_to_axum_request(lambda_request);
    info!("Converted to axum request: {} {}", axum_request.method(), axum_request.uri());

    // Process the request with axum
    let axum_response = app
        .oneshot(axum_request)
        .await
        .unwrap_or_else(|err| {
            eprintln!("Error processing request: {}", err);
            axum::http::Response::builder()
                .status(500)
                .body(Body::from("Internal Server Error"))
                .unwrap()
        });

    // Convert axum response to lambda response
    Ok(axum_to_lambda_response(axum_response).await)
}

// Main function for Lambda
#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    // Initialize tracing for AWS Lambda
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    // Run the Lambda handler
    run(service_fn(handler)).await
}
