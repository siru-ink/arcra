use sqlx::{
    migrate::{MigrateError, Migrator},
    sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions},
};
use std::{path::Path, str::FromStr, time::Duration};

mod record;

async fn apply_db_migrations(pool: &SqlitePool) -> Result<(), MigrateError> {
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(pool).await
}

pub async fn init_db_connection() -> SqlitePool {
    let connection_options = SqliteConnectOptions::from_str("sqlite:data.db")
        .expect("Compiled slite connection string should produce functioning db")
        .create_if_missing(true)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await
        .expect("Database should be reachable.");

    if let Err(e) = apply_db_migrations(&pool).await {
        println!("Error applying db migrations: {}", e);
    };

    pool
}
