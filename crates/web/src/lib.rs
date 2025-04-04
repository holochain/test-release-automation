use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;
use test_ra_core::{calculate_fibonacci, get_default_config, greet, AppConfig};

pub async fn run_server(port: u16) {
    let app = Router::new()
        .route("/", get(|| async { "Welcome to the web API!" }))
        .route("/greet/:name", get(greet_handler))
        .route("/config", get(config_handler))
        .route("/config/update", post(update_config))
        .route("/fibonacci/:count", get(fibonacci_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn greet_handler(Path(name): Path<String>) -> String {
    greet(&name)
}

async fn config_handler() -> Json<AppConfig> {
    Json(get_default_config())
}

async fn update_config(Json(config): Json<AppConfig>) -> Json<AppConfig> {
    // In a real app, you would save the config somewhere
    println!("Updating config: {:?}", config);
    Json(config)
}

async fn fibonacci_handler(Path(count): Path<u32>) -> Json<Vec<u32>> {
    Json(calculate_fibonacci(count))
}
