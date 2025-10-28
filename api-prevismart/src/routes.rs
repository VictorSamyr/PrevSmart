use crate::handlers::user::login;
use crate::models::AppState;
use axum::routing::post;
use axum::{Json, Router};

pub async fn router() -> Router<AppState> {
    Router::new().route("/login", post(login))
}
