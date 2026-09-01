use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::env;

pub async fn conn() -> Pool<Postgres> {
    dotenv().ok();

    let url =
        env::var("DATABASE_URL").expect("A variável de ambiente DATABASE_URL não foi encontrada");

    let pool = sqlx::postgres::PgPool::connect(&url)
        .await
        .unwrap_or_else(|error| {
            panic!("Falha ao conectar em {url}. Erro: {error:?}");
        });

    pool
}
