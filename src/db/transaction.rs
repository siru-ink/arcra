use sqlx::SqliteConnection;

use crate::domain;

#[derive(Debug, thiserror::Error)]
pub enum CreateTransactionError {
    #[error("Failure in sqlx database connection.")]
    SqlxError(#[from] sqlx::Error),
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

impl crate::db::Db {
    pub fn create_transaction(
        conn: &mut SqliteConnection,
        transaction: domain::NewTransaction,
    ) -> Result<domain::Transaction, CreateTransactionError> {
        todo!()
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
