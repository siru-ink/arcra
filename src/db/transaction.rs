use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{SqlitePool, query, query_as};

use crate::db::Account;

pub struct Transaction {
    id: i64,
    description: String,
    time: DateTime<Utc>,
    credit_account: String,
    credit_amount: i64,
    credit_currency: String,
    debit_account: String,
    debit_amount: i64,
    debit_currency: String,
}

pub struct TransactionBalances {
    credit_account: i64,
    credit_amount: i64,
    debit_account: i64,
    debit_amount: i64,
}

impl Transaction {
    pub async fn drop(pool: &SqlitePool, id: i64) -> bool {
        let balances = match query_as!(
            TransactionBalances,
            "SELECT credit_account, credit_amount, debit_account, debit_amount FROM transactions WHERE id = ?",
            id
        )
        .fetch_one(pool)
        .await
        {
            Ok(bal) => bal,
            Err(_) => return false,
        };

        let increase =
            Account::increase_balance(pool, balances.credit_account, balances.credit_amount).await;
        let decrease =
            Account::decrease_balance(pool, balances.debit_account, balances.debit_amount).await;

        if increase && decrease {
            return query!("DELETE FROM transactions WHERE id = ?", id)
                .execute(pool)
                .await
                .is_ok();
        }

        false
    }

    pub async fn get(pool: &SqlitePool, id: i64) -> Option<Transaction> {
        query_as!(
            Transaction,
            "SELECT
                t.id            as id,
                t.description   as description,
                t.time          as \"time: _\",
                ca.name         as credit_account,
                t.credit_amount as credit_amount,
                cforca.symbol   as credit_currency,
                da.name         as debit_account,
                t.debit_amount  as debit_amount,
                cforda.symbol   as debit_currency
             FROM transactions t
             JOIN accounts ca on t.credit_account = ca.id
             JOIN accounts da on t.debit_account = da.id
             JOIN currencies cforca on ca.currency = cforca.id
             JOIN currencies cforda on da.currency = cforda.id
             WHERE t.id = ?",
            id
        )
        .fetch_optional(pool)
        .await
        .ok()?
    }

    pub async fn list_all(pool: &SqlitePool) -> Vec<Transaction> {
        match query_as!(
            Transaction,
            "SELECT
                t.id            as id,
                t.description   as description,
                t.time          as \"time: _\",
                ca.name         as credit_account,
                t.credit_amount as credit_amount,
                cforca.symbol   as credit_currency,
                da.name         as debit_account,
                t.debit_amount  as debit_amount,
                cforda.symbol   as debit_currency
             FROM transactions t
             JOIN accounts ca on t.credit_account = ca.id
             JOIN accounts da on t.debit_account = da.id
             JOIN currencies cforca on ca.currency = cforca.id
             JOIN currencies cforda on da.currency = cforda.id"
        )
        .fetch_all(pool)
        .await
        {
            Ok(records) => records,
            Err(e) => {
                eprintln!("Transactions could not be retrieved from db: {}", e);
                vec![]
            }
        }
    }

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

#[derive(Serialize)]
pub struct TransactionsRoundedCurrencyValues {
    id: i64,
    description: String,
    time: DateTime<Utc>,
    credit_account: String,
    credit_amount: f64,
    credit_currency: String,
    debit_account: String,
    debit_amount: f64,
    debit_currency: String,
}

impl TransactionsRoundedCurrencyValues {
    pub fn from(transactions: Vec<Transaction>) -> Vec<TransactionsRoundedCurrencyValues> {
        transactions
            .into_iter()
            .map(|t| TransactionsRoundedCurrencyValues {
                id: t.id,
                description: t.description,
                time: t.time,
                credit_account: t.credit_account,
                credit_amount: t.credit_amount as f64 / 100.0,
                credit_currency: t.credit_currency,
                debit_account: t.debit_account,
                debit_amount: t.debit_amount as f64 / 100.0,
                debit_currency: t.debit_currency,
            })
            .collect()
    }
}
