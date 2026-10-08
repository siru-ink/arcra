use serde::Serialize;
use sqlx::{QueryBuilder, SqlitePool, query, query_as};

#[derive(Serialize)]
pub struct Account {
    id: i64,
    name: String,
    currency: String,
    balance: i64,
    account_type: String,
}

impl Account {
    pub async fn increase_balance(pool: &SqlitePool, account_id: i64, amount: i64) -> bool {
        struct Balance {
            balance: i64,
        }

        let account_balance = match query_as!(
            Balance,
            "SELECT balance FROM accounts WHERE id = ?",
            account_id
        )
        .fetch_one(pool)
        .await
        {
            Ok(balance) => balance,
            Err(_) => return false,
        };

        query!(
            "UPDATE accounts SET balance = ? WHERE id = ?",
            (account_balance.balance + amount),
            account_id
        )
        .execute(pool)
        .await
        .is_ok()
    }

    pub async fn decrease_balance(pool: &SqlitePool, account_id: i64, amount: i64) -> bool {
        struct Balance {
            balance: i64,
        }

        let account_balance = match query_as!(
            Balance,
            "SELECT balance FROM accounts WHERE id = ?",
            account_id
        )
        .fetch_one(pool)
        .await
        {
            Ok(balance) => balance,
            Err(_) => return false,
        };

        query!(
            "UPDATE accounts SET balance = ? WHERE id = ?",
            (account_balance.balance - amount),
            account_id
        )
        .execute(pool)
        .await
        .is_ok()
    }

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

    pub async fn drop_all(pool: &SqlitePool, ids: &Vec<i64>) {
        let mut sql_builder = QueryBuilder::new("DELETE FROM accounts WHERE id IN (");
        let mut separator = sql_builder.separated(", ");
        for id in ids {
            separator.push_bind(id);
        }
        separator.push_unseparated(")");

        let _ = sql_builder.build().execute(pool).await;
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }
}
