use crate::domain;
use sqlx::SqlitePool;

#[derive(Debug, thiserror::Error)]
pub enum CreateNewTransactionError {}

#[derive(Debug, thiserror::Error)]
pub enum DropTransactionError {
    #[error("Failed to convert id into TransactionID domain object.")]
    FailedTransactionIDConversion(#[from] domain::CreateTransactionIDError),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadTransactionError {}

#[derive(Debug, thiserror::Error)]
pub enum ListAllTransactionsError {}

impl domain::Transaction {
    pub async fn drop(dbase: &SqlitePool, transaction_id: i64) -> Result<(), DropTransactionError> {
        let transaction_id = domain::TransactionID::try_from(transaction_id)?;

        todo!()
    }

    pub async fn get_by_id(
        dbase: &SqlitePool,
        transaction_id: i64,
    ) -> Result<Self, ReadTransactionError> {
        todo!()
    }

    pub async fn list_all(dbase: &SqlitePool) -> Result<Vec<Self>, ListAllTransactionsError> {
        todo!()
    }

    pub async fn new(
        dbase: &SqlitePool,
        description: String,
        credit_account_id: i64,
        debit_account_id: i64,
        credit_amount: f64,
        debit_amount: Option<f64>,
    ) -> Result<Self, CreateNewTransactionError> {
        todo!()
    }
}
