use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn get_connection_pool() -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://shyrio:password@127.0.0.1:5433/mochi")
        .await
}
