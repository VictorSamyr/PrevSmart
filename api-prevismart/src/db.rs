use sqlx::{Pool, Postgres};

pub async fn conn(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    sqlx::postgres::PgPool::connect(database_url).await
}
