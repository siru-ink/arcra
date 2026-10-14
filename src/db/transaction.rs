use sqlx::SqliteConnection;

use crate::domain;

#[derive(Debug, thiserror::Error)]
pub enum CreateTransactionError {
    #[error("Failure in sqlx database connection.")]
    SqlxError(#[from] sqlx::Error),
    #[error("Could not convert database row into Transaction domain object.")]
    HydrateError(#[from] domain::HydrateTransactionError),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadTransactionError {
    #[error("Failure in sqlx database connection.")]
    SqlxError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateTransactionError {
    #[error("Failure in sqlx database connection.")]
    SqlxError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteTransactionError {
    #[error("Failure in sqlx database connection.")]
    SqlxError(#[from] sqlx::Error),
}

pub struct TransactionRow {
    id: i64,
    description: String,
    time: chrono::DateTime<chrono::Utc>,
    credit_account_id: i64,
    credit_account_name: String,
    credit_account_currency_id: i64,
    credit_account_currency_name: String,
    credit_account_currency_descriptor: String,
    credit_account_currency_symbol: String,
    credit_account_balance: i64,
    credit_account_account_type: i64,
    credit_amount: i64,
    debit_account_id: i64,
    debit_account_name: String,
    debit_account_currency_id: i64,
    debit_account_currency_name: String,
    debit_account_currency_descriptor: String,
    debit_account_currency_symbol: String,
    debit_account_balance: i64,
    debit_account_account_type: i64,
    debit_amount: i64,
}

impl TryFrom<TransactionRow> for domain::Transaction {
    type Error = domain::HydrateTransactionError;

    fn try_from(value: TransactionRow) -> Result<Self, Self::Error> {
        Self::hydrate(
            value.id,
            value.description,
            value.time,
            value.credit_account_id,
            value.credit_account_name,
            value.credit_account_currency_id,
            value.credit_account_currency_name,
            value.credit_account_currency_descriptor,
            value.credit_account_currency_symbol,
            value.credit_account_balance,
            value.credit_account_account_type,
            value.credit_amount,
            value.debit_account_id,
            value.debit_account_name,
            value.debit_account_currency_id,
            value.debit_account_currency_name,
            value.debit_account_currency_descriptor,
            value.debit_account_currency_symbol,
            value.debit_account_balance,
            value.debit_account_account_type,
            value.debit_amount,
        )
    }
}

impl crate::db::Db {
    pub async fn create_transaction(
        conn: &mut SqliteConnection,
        transaction: domain::NewTransaction,
    ) -> Result<domain::Transaction, CreateTransactionError> {
        let new_id = sqlx::query!(
            r#"INSERT INTO transactions (description, time, credit_account, credit_amount, debit_account, debit_amount)
               VALUES (?, ?, ?, ?, ?, ?)
               RETURNING id as "id!""#,
            transaction.description(),
            transaction.time(),
            transaction.credit_account().id(),
            transaction.credit_amount(),
            transaction.debit_account().id(),
            transaction.debit_amount()
        )
        .fetch_one(&mut *conn)
        .await?
        .id;

        let new_transaction = sqlx::query_as!(
            TransactionRow,
            r#"SELECT
                 t.id as id,
                 t.description as description,
                 t.time as "time: _",
                 ca.id as credit_account_id,
                 ca.name as credit_account_name,
                 cc.id as credit_account_currency_id,
                 cc.name as credit_account_currency_name,
                 cc.descriptor as credit_account_currency_descriptor,
                 cc.symbol as credit_account_currency_symbol,
                 ca.balance as credit_account_balance,
                 ca.account_type as credit_account_account_type,
                 t.credit_amount as credit_amount,
                 da.id as debit_account_id,
                 da.name as debit_account_name,
                 dc.id as debit_account_currency_id,
                 dc.name as debit_account_currency_name,
                 dc.descriptor as debit_account_currency_descriptor,
                 dc.symbol as debit_account_currency_symbol,
                 da.balance as debit_account_balance,
                 da.account_type as debit_account_account_type,
                 t.debit_amount as debit_amount
               FROM transactions t
               JOIN accounts ca on t.credit_account = ca.id
               JOIN currencies cc on ca.currency = cc.id
               JOIN accounts da on t.debit_account = da.id
               JOIN currencies dc on da.currency = dc.id
               WHERE t.id = ?"#,
            new_id
        )
        .fetch_one(conn)
        .await?;

        Ok(new_transaction.try_into()?)
    }

    pub fn read_transaction(
        conn: &mut SqliteConnection,
        id: domain::TransactionID,
    ) -> Result<Option<domain::Transaction>, ReadTransactionError> {
        todo!()
    }

    pub fn update_transaction(
        conn: &mut SqliteConnection,
        id: domain::TransactionID,
    ) -> Result<(), UpdateTransactionError> {
        todo!()
    }

    pub fn delete_transaction(
        conn: &mut SqliteConnection,
        id: domain::TransactionID,
    ) -> Result<(), DeleteTransactionError> {
        todo!()
    }
}
