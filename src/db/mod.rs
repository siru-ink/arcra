use sqlx::{
    migrate::{MigrateError, Migrator},
    sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions},
};
use std::{path::Path, str::FromStr, time::Duration};

mod account;
mod account_type;
mod currency;
mod record;
mod session;

pub use account::Account;
pub use account_type::AccountType;
pub use currency::Currency;
pub use session::Session;

async fn apply_db_migrations(pool: &SqlitePool) -> Result<(), MigrateError> {
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(pool).await
}

pub async fn init_db_connection() -> SqlitePool {
    let connection_options = SqliteConnectOptions::from_str("sqlite:./data/data.db")
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
