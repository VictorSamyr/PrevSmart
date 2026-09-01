use axum::http::{HeaderValue, Method, header};
use dotenv::dotenv;
use std::{env, error::Error, fmt, net::IpAddr};
use tower_http::cors::CorsLayer;

pub struct Config {
    pub database_url: String,
    pub host: IpAddr,
    pub port: u16,
    pub cors_allowed_origin: HeaderValue,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingVariable(&'static str),
    InvalidDatabaseUrl,
    InvalidHost,
    InvalidPort,
    InvalidCorsOrigin,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        let database_url = required_variable("DATABASE_URL")?;
        if database_url.trim().is_empty() {
            return Err(ConfigError::InvalidDatabaseUrl);
        }

        let host = required_variable("APP_HOST")?
            .parse()
            .map_err(|_| ConfigError::InvalidHost)?;

        let port = required_variable("APP_PORT")?
            .parse()
            .map_err(|_| ConfigError::InvalidPort)?;

        let cors_origin = required_variable("CORS_ALLOWED_ORIGIN")?;
        let cors_allowed_origin = parse_cors_origin(&cors_origin)?;

        Ok(Self {
            database_url,
            host,
            port,
            cors_allowed_origin,
        })
    }

    pub fn cors_layer(&self) -> CorsLayer {
        CorsLayer::new()
            .allow_origin(self.cors_allowed_origin.clone())
            .allow_headers([header::CONTENT_TYPE])
            .allow_methods([Method::POST])
    }
}

fn required_variable(name: &'static str) -> Result<String, ConfigError> {
    env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or(ConfigError::MissingVariable(name))
}

fn parse_cors_origin(origin: &str) -> Result<HeaderValue, ConfigError> {
    if origin == "*" || !(origin.starts_with("http://") || origin.starts_with("https://")) {
        return Err(ConfigError::InvalidCorsOrigin);
    }

    HeaderValue::from_str(origin).map_err(|_| ConfigError::InvalidCorsOrigin)
}

impl fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingVariable(name) => {
                write!(formatter, "a variável de ambiente {name} não foi definida")
            }
            Self::InvalidDatabaseUrl => {
                write!(formatter, "DATABASE_URL deve conter um valor não vazio")
            }
            Self::InvalidHost => write!(formatter, "APP_HOST deve conter um endereço IP válido"),
            Self::InvalidPort => write!(formatter, "APP_PORT deve conter uma porta válida"),
            Self::InvalidCorsOrigin => write!(
                formatter,
                "CORS_ALLOWED_ORIGIN deve conter uma origem HTTP ou HTTPS específica"
            ),
        }
    }
}

impl Error for ConfigError {}
