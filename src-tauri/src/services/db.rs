use sqlx::pool::PoolOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};

pub async fn init_pool() -> Result<SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .connect("sqlite::memory:")
        .await
}
