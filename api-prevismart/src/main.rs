use axum::serve;
use std::{error::Error, fmt, net::SocketAddr};

use crate::{config::Config, db::conn, models::AppState};

mod config;
mod db;
mod handlers;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("Falha ao iniciar a API: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), StartupError> {
    let config = Config::from_env()?;

    let cors = config.cors_layer();

    let db_conn = conn(&config.database_url)
        .await
        .map_err(|_| StartupError::DatabaseConnection)?;

    let app_state = AppState { pool: db_conn };

    let app = routes::router().await.layer(cors).with_state(app_state);

    let address = SocketAddr::new(config.host, config.port);
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .map_err(|_| StartupError::BindAddress)?;

    println!("API disponível em http://{address}");

    serve(listener, app).await.map_err(|_| StartupError::Server)
}

#[derive(Debug)]
enum StartupError {
    Configuration(config::ConfigError),
    DatabaseConnection,
    BindAddress,
    Server,
}

impl From<config::ConfigError> for StartupError {
    fn from(error: config::ConfigError) -> Self {
        Self::Configuration(error)
    }
}

impl fmt::Display for StartupError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(error) => error.fmt(formatter),
            Self::DatabaseConnection => {
                write!(formatter, "não foi possível conectar ao banco de dados")
            }
            Self::BindAddress => write!(
                formatter,
                "não foi possível usar o host e a porta configurados"
            ),
            Self::Server => write!(
                formatter,
                "o servidor foi encerrado devido a um erro interno"
            ),
        }
    }
}

impl Error for StartupError {}
