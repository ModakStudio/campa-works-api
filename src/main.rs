use axum::Router;

use tokio::net::TcpListener;

use std::sync::Arc;

mod config;
mod db;
mod dto;
mod error;
mod handler;
mod models;
mod repository;
mod service;
mod router;
mod schema;
mod state;
mod utils;

use state::app_state::AppState;
use db::pool::create_pool;

use crate::router::user_router::user_router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = create_pool();
    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .nest("/api/users", user_router())
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
