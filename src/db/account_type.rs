use serde::Serialize;
use sqlx::{SqlitePool, query_as};

#[derive(Serialize)]
pub struct AccountType {
    id: i64,
    name: String,
    balance_type: String,
}

impl AccountType {
    pub async fn list_all(pool: &SqlitePool) -> Vec<AccountType> {
        match query_as!(AccountType, "SELECT * FROM account_types")
            .fetch_all(pool)
            .await
        {
            Ok(account_types) => account_types,
            Err(e) => {
                eprintln!("Failed to access account_types in database: {}", e);
                vec![]
            }
        }
    }
}
