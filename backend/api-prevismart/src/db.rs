use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::env;
use std::error::Error;

pub async fn conn() -> Result<Pool<Postgres>, Box<dyn Error>> {
    dotenv().ok();

    let binding: Result<String, std::env::VarError> = env::var("DATABASE_URL");
    let url: &str = match binding {
        Ok(ref s) => s,
        Err(error) => panic!("Erro ao Obter URL do Banco de Dados: {error:?}"),
    };

    let pool: Pool<Postgres> = match sqlx::postgres::PgPool::connect(url).await {
        Ok(p) => p,
        Err(error) => panic!("Erro ao Conectar ao Banco de Dados: {error:?}"),
    };

    Ok(pool)
}
