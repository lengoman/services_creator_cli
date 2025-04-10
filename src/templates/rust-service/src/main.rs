use {{project-name}}::common::validation::RapidApiConfig;
use {{project-name}}::routes::create_router;
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tower_http::cors::CorsLayer;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 3010)]
    port: u16,

    /// Number of worker threads
    #[arg(short, long)]
    workers: Option<usize>,
}

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Parse command line arguments
    let args = Args::parse();

    // Set up multi-threaded runtime
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.workers.unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|p| p.get())
                .unwrap_or(1)
        }))
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    runtime.block_on(async {
        // Configure RapidAPI settings from environment variables
        let rapidapi_config = Arc::new(
            RapidApiConfig::from_env()
                .expect("Failed to load RapidAPI configuration from environment variables")
        );

        // Create router
        let app = create_router(rapidapi_config)
            .layer(CorsLayer::permissive());

        // Create TCP listener
        let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        println!("Listening on {}", addr);

        // Start server
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    });
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl+c");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Shutting down server...");
}
