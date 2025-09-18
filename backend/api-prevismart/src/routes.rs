use crate::db::conn;
use crate::models::Payloads;
use axum::Router;
use axum::routing::{get, post};
use sqlx::{Pool, Postgres};
use std::error::Error;

pub async fn router() -> Router {
    let pool: Result<Pool<Postgres>, Box<dyn Error>> = conn().await;
    Router::new().route("/", get(|| async { "Hello, World!" }))
    //.route("/login", post(handler))
}
