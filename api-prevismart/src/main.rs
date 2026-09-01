use axum::serve;
use tower_http::cors::{Any, CorsLayer};

use crate::{db::conn, models::AppState};

mod db;
mod handlers;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let db_conn = conn().await;

    let app_state = AppState { pool: db_conn };

    let app = routes::router().await.layer(cors).with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
