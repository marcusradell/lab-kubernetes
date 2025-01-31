use sqlx::postgres::PgPool;
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Environment variable error: {0}")]
    Environment(#[from] env::VarError),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub async fn create_pool() -> Result<PgPool, DatabaseError> {
    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPool::connect(&database_url).await?;

    sqlx::query("SELECT 1").fetch_one(&pool).await?;

    Ok(pool)
}
