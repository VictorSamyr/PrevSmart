use axum::serve;

mod db;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {

    let app = routes::router().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}
