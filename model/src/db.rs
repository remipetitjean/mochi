use sea_orm::{Database, DatabaseConnection};

pub async fn get_database_connection() -> DatabaseConnection {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    Database::connect(db_url)
        .await
        .expect("Database connection failed")
}
