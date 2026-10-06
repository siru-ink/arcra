use serde::Serialize;
use sqlx::{SqlitePool, query_as};

#[derive(Serialize)]
pub struct Account {
    id: i64,
    name: String,
    currency: String,
    balance: i64,
    account_type: String,
}

impl Account {
    pub async fn list_all(pool: &SqlitePool) -> Vec<Account> {
        match query_as!(
            Account,
            "SELECT accounts.id as id,
                    accounts.name as name,
                    currencies.descriptor as currency,
                    accounts.balance as balance,
                    account_types.name as account_type
            FROM accounts
            JOIN currencies on accounts.currency = currencies.id
            JOIN account_types on accounts.account_type = account_types.id"
        )
        .fetch_all(pool)
        .await
        {
            Ok(accounts) => accounts,
            Err(e) => {
                eprintln!("Failed to access accounts in database: {}", e);
                return vec![];
            }
        }
    }
}
