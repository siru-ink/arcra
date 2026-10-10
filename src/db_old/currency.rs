use serde::Serialize;
use sqlx::{SqlitePool, query, query_as};

#[derive(Serialize)]
pub struct Currency {
    id: i64,
    name: String,
    descriptor: String,
    symbol: String,
}

impl Currency {
    pub async fn list_all(pool: &SqlitePool) -> Vec<Currency> {
        match query_as!(Currency, "SELECT * FROM currencies")
            .fetch_all(pool)
            .await
        {
            Ok(currencies) => currencies,
            Err(e) => {
                eprintln!("Failed to access currencies in database: {}", e);
                vec![]
            }
        }
    }

    pub async fn new(pool: &SqlitePool, name: &str, descriptor: &str, symbol: &str) -> bool {
        query_as!(
            Currency,
            "INSERT INTO currencies (name, descriptor, symbol)
             VALUES (?,?,?)
             RETURNING id,name,descriptor,symbol",
            name,
            descriptor,
            symbol
        )
        .fetch_optional(pool)
        .await
        .is_ok()
    }

    pub async fn delete(pool: &SqlitePool, id: i64) -> bool {
        query!("DELETE FROM currencies WHERE id = ?", id)
            .execute(pool)
            .await
            .is_ok()
    }
}
