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
    #[error("Could not convert database row into Transaction domain object.")]
    HydrateError(#[from] domain::HydrateTransactionError),
}

// #[derive(Debug, thiserror::Error)]
// pub enum UpdateTransactionError {
//     #[error("Failure in sqlx database connection.")]
//     SqlxError(#[from] sqlx::Error),
// }

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
    category_id: i64,
    category_name: String,
    category_description: String,
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
            value.category_id,
            value.category_name,
            value.category_description,
        )
    }
}

impl crate::db::Db {
    pub async fn create_transaction(
        conn: &mut SqliteConnection,
        transaction: domain::NewTransaction,
    ) -> Result<domain::Transaction, CreateTransactionError> {
        let new_id = sqlx::query!(
            r#"INSERT INTO transactions (description, time, credit_account, credit_amount, debit_account, debit_amount, category)
               VALUES (?, ?, ?, ?, ?, ?, ?)
               RETURNING id as "id!""#,
            transaction.description(),
            transaction.time(),
            transaction.credit_account().id(),
            transaction.credit_amount(),
            transaction.debit_account().id(),
            transaction.debit_amount(),
            transaction.category().id()
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
                 ccu.id as credit_account_currency_id,
                 ccu.name as credit_account_currency_name,
                 ccu.descriptor as credit_account_currency_descriptor,
                 ccu.symbol as credit_account_currency_symbol,
                 ca.balance as credit_account_balance,
                 ca.account_type as credit_account_account_type,
                 t.credit_amount as credit_amount,
                 da.id as debit_account_id,
                 da.name as debit_account_name,
                 dcu.id as debit_account_currency_id,
                 dcu.name as debit_account_currency_name,
                 dcu.descriptor as debit_account_currency_descriptor,
                 dcu.symbol as debit_account_currency_symbol,
                 da.balance as debit_account_balance,
                 da.account_type as debit_account_account_type,
                 t.debit_amount as debit_amount,
                 cat.id as category_id,
                 cat.name as category_name,
                 cat.description as category_description
               FROM transactions t
               JOIN accounts ca on t.credit_account = ca.id
               JOIN currencies ccu on ca.currency = ccu.id
               JOIN accounts da on t.debit_account = da.id
               JOIN currencies dcu on da.currency = dcu.id
               JOIN categories cat on t.category = cat.id
               WHERE t.id = ?"#,
            new_id
        )
        .fetch_one(conn)
        .await?;

        Ok(new_transaction.try_into()?)
    }

    pub async fn read_transaction(
        conn: &mut SqliteConnection,
        id: domain::TransactionID,
    ) -> Result<Option<domain::Transaction>, ReadTransactionError> {
        let transaction_row = sqlx::query_as!(
            TransactionRow,
            r#"SELECT
                 t.id as id,
                 t.description as description,
                 t.time as "time: _",
                 ca.id as credit_account_id,
                 ca.name as credit_account_name,
                 ccu.id as credit_account_currency_id,
                 ccu.name as credit_account_currency_name,
                 ccu.descriptor as credit_account_currency_descriptor,
                 ccu.symbol as credit_account_currency_symbol,
                 ca.balance as credit_account_balance,
                 ca.account_type as credit_account_account_type,
                 t.credit_amount as credit_amount,
                 da.id as debit_account_id,
                 da.name as debit_account_name,
                 dcu.id as debit_account_currency_id,
                 dcu.name as debit_account_currency_name,
                 dcu.descriptor as debit_account_currency_descriptor,
                 dcu.symbol as debit_account_currency_symbol,
                 da.balance as debit_account_balance,
                 da.account_type as debit_account_account_type,
                 t.debit_amount as debit_amount,
                 cat.id as category_id,
                 cat.name as category_name,
                 cat.description as category_description
               FROM transactions t
               JOIN accounts ca on t.credit_account = ca.id
               JOIN currencies ccu on ca.currency = ccu.id
               JOIN accounts da on t.debit_account = da.id
               JOIN currencies dcu on da.currency = dcu.id
               JOIN categories cat on t.category = cat.id
               WHERE t.id = ?"#,
            i64::from(id)
        )
        .fetch_optional(conn)
        .await?;

        Ok(transaction_row
            .map(domain::Transaction::try_from)
            .transpose()?)
    }

    //     pub async fn update_transaction(
    //         conn: &mut SqliteConnection,
    //         transaction: domain::Transaction,
    //     ) -> Result<(), UpdateTransactionError> {
    //         let _ = sqlx::query!(
    //             "UPDATE transactions
    //              SET
    //                description = ?,
    //                time = ?,
    //                credit_account = ?,
    //                credit_amount = ?,
    //                debit_account = ?,
    //                debit_amount = ?
    //              WHERE id = ?",
    //             transaction.description(),
    //             transaction.time(),
    //             transaction.credit_account().id(),
    //             transaction.credit_amount(),
    //             transaction.debit_account().id(),
    //             transaction.debit_amount(),
    //             transaction.id()
    //         )
    //         .execute(conn)
    //         .await?;
    //
    //         Ok(())
    //     }

    pub async fn delete_transaction(
        conn: &mut SqliteConnection,
        id: domain::TransactionID,
    ) -> Result<(), DeleteTransactionError> {
        let _ = sqlx::query!("DELETE FROM transactions WHERE id = ?", i64::from(id))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn read_all_transactions(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<domain::Transaction>, ReadTransactionError> {
        let transaction_rows = sqlx::query_as!(
            TransactionRow,
            r#"SELECT
                 t.id as id,
                 t.description as description,
                 t.time as "time: _",
                 ca.id as credit_account_id,
                 ca.name as credit_account_name,
                 ccu.id as credit_account_currency_id,
                 ccu.name as credit_account_currency_name,
                 ccu.descriptor as credit_account_currency_descriptor,
                 ccu.symbol as credit_account_currency_symbol,
                 ca.balance as credit_account_balance,
                 ca.account_type as credit_account_account_type,
                 t.credit_amount as credit_amount,
                 da.id as debit_account_id,
                 da.name as debit_account_name,
                 dcu.id as debit_account_currency_id,
                 dcu.name as debit_account_currency_name,
                 dcu.descriptor as debit_account_currency_descriptor,
                 dcu.symbol as debit_account_currency_symbol,
                 da.balance as debit_account_balance,
                 da.account_type as debit_account_account_type,
                 t.debit_amount as debit_amount,
                 cat.id as category_id,
                 cat.name as category_name,
                 cat.description as category_description
               FROM transactions t
               JOIN accounts ca on t.credit_account = ca.id
               JOIN currencies ccu on ca.currency = ccu.id
               JOIN accounts da on t.debit_account = da.id
               JOIN currencies dcu on da.currency = dcu.id
               JOIN categories cat on t.category = cat.id"#
        )
        .fetch_all(conn)
        .await?;

        let transactions: Vec<domain::Transaction> = transaction_rows
            .into_iter()
            .map(domain::Transaction::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(transactions)
    }
}
