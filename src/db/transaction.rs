use chrono::Utc;
use sqlx::{SqlitePool, query};

pub struct Transaction {}

impl Transaction {
    pub async fn new(
        pool: &SqlitePool,
        description: &str,
        credit_account: i64,
        credit_amount: i64,
        debit_account: i64,
        debit_amount: i64,
    ) -> bool {
        query!(
            "INSERT INTO transactions (description, time, credit_account, credit_amount, debit_account, debit_amount)
             VALUES (?, ?, ?, ?, ?, ?)",
            description,
            Utc::now(),
            credit_account,
            credit_amount,
            debit_account,
            debit_amount
        ).execute(pool).await.is_ok()
    }
}
