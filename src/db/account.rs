use serde::Serialize;
use sqlx::{SqlitePool, query, query_as};

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
                    currencies.name as currency,
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

    pub async fn new(pool: &SqlitePool, name: &str, currency: i64, account_type: i64) {
        if let Err(e) = query!(
            "INSERT INTO accounts (name, currency, balance, account_type) VALUES (?,?,0,?)",
            name,
            currency,
            account_type
        )
        .execute(pool)
        .await
        {
            #[cfg(debug_assertions)]
            eprintln!("Could not create new account: {}", e);
        }
    }

    pub async fn get(pool: &SqlitePool, id: i64) -> Option<Account> {
        query_as!(
            Account,
            "SELECT accounts.id as id,
                    accounts.name as name,
                    currencies.name as currency,
                    accounts.balance as balance,
                    account_types.name as account_type
             FROM accounts
             JOIN currencies on accounts.currency = currencies.id
             JOIN account_types on accounts.account_type = account_types.id
             WHERE accounts.id = ?",
            id
        )
        .fetch_optional(pool)
        .await
        .ok()?
    }

    pub async fn rename(&self, pool: &SqlitePool, new_name: &str) {
        if self.name != new_name {
            let _ = query!(
                "UPDATE accounts SET name = ? WHERE id = ?",
                new_name,
                self.id
            )
            .execute(pool)
            .await;
        }
    }
}
