use sqlx::{SqlitePool, query_as};

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
}
