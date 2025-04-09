use rust_qr_generator::{
    common::validation::RapidApiConfig,
    routes::create_router,
};
use clap::Parser;
use std::net::SocketAddr;
use std::sync::Arc;
use num_cpus::get;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3010)]
    port: u16,
    workers: Option<usize>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let num_workers = args.workers.unwrap_or_else(num_cpus::get);

    let _runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_workers)
        .enable_all()
        .build()
        .unwrap();

    // Configure RapidAPI settings
    let rapidapi_config = Arc::new(RapidApiConfig::new(
        "6f79849e04msh105d56a90bc7568p11ccd6jsn0c8c9c6fde05",
        "059fe2c0-adcd-11ef-9c3f-a75c3ffbbfe4",
        "qr-code-generator-logo.p.rapidapi.com"
    ));

    let app = create_router(rapidapi_config);

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap_or_else(|e| {
        eprintln!("Failed to bind to port {}: {}", args.port, e);
        std::process::exit(1);
    });

    println!("Server running on {} with {} worker threads", addr, num_workers);

    let server = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .with_graceful_shutdown(shutdown_signal())
        .tcp_nodelay(true);

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    println!("Shutting down server...");
}
