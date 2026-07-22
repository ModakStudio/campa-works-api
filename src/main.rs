use axum::{Router, routing::get};

use tokio::net::TcpListener;

mod app_state;
mod config;
mod db;

use app_state::AppState;
use db::pool::create_pool;

async fn test_api() -> &'static str {
    "Server is running"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = create_pool();
    let state = AppState { pool };

    let app = Router::new()
        .route("/test", get(test_api))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
