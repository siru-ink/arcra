use crate::{db, domain};
use sqlx::SqlitePool;

#[derive(Debug, thiserror::Error)]
pub enum CreateNewTransactionError {
    #[error("Transaction must reference two different accounts.")]
    AccountsMustDiffer,
    #[error("Provided credit account id was not valid.")]
    InvalidCreditAccountID(domain::CreateAccountIDError),
    #[error("Provided debit account id was not valid.")]
    InvalidDebitAccountID(domain::CreateAccountIDError),
    #[error("Failed to aquire a database connection.")]
    DBConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to read credit account information from database.")]
    DBReadCreditAccountFailed(db::ReadAccountError),
    #[error("Failed to read debit account information from database.")]
    DBReadDebitAccountFailed(db::ReadAccountError),
    #[error("No account found for given credit account id.")]
    CreditAccountNotFound,
    #[error("No account found for given debit account id.")]
    DebitAccountNotFound,
    #[error("Failed to create a NewTransaction domain object.")]
    CreateNewTransactionFailed(#[from] domain::CreateNewTransactionError),
    #[error("Failed to create transaction record in database.")]
    DBCreateTransactionFailed(#[from] db::CreateTransactionError),
    #[error("Failed to write sql transaction to database.")]
    DBSqlTransactionCommitFailed(sqlx::Error),
    #[error("Failed to update the balance in the credit account record in the database.")]
    DBUpdateCreditAccountFailed(db::UpdateAccountError),
    #[error("Failed to update the balance in the debit account record in the database.")]
    DBUpdateDebitAccountFailed(db::UpdateAccountError),
}

#[derive(Debug, thiserror::Error)]
pub enum DropTransactionError {
    #[error("Failed to convert id into TransactionID domain object.")]
    FailedTransactionIDConversion(#[from] domain::CreateTransactionIDError),
    #[error("Failed to aquire a database connection.")]
    DBConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to commit sql transaction to database.")]
    DBCommitFailed(sqlx::Error),
    #[error("Failed to delete transaction record in database.")]
    DBTransactionDeleteFailed(#[from] db::DeleteTransactionError),
    #[error("Failed to read transaction record from database.")]
    DBTransactionReadFailed(#[from] db::ReadTransactionError),
    #[error("No transaction found in database for given id.")]
    DBTransactionNotFound,
    #[error("Failed to write updated account balances to database.")]
    DBUpdateAccountsFailed(#[from] db::UpdateAccountError),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadTransactionError {
    #[error("Failed to convert id into TransactionID domain object.")]
    FailedTransactionIDConversion(#[from] domain::CreateTransactionIDError),
    #[error("Failed to aquire a database connection.")]
    DBConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to read transaction data from database.")]
    DBReadFailed(#[from] db::ReadTransactionError),
    #[error("No transaction found in database for given id.")]
    DBTransactionNotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum ListAllTransactionsError {
    #[error("Failed to aquire a database connection.")]
    DBConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to read transaction data from database.")]
    DBReadFailed(#[from] db::ReadTransactionError),
}

impl domain::Transaction {
    pub async fn drop(dbase: &SqlitePool, transaction_id: i64) -> Result<(), DropTransactionError> {
        let transaction_id = domain::TransactionID::try_from(transaction_id)?;
        let mut conn = dbase.begin().await?;
        let Some(transaction) = db::Db::read_transaction(&mut conn, transaction_id).await? else {
            return Err(DropTransactionError::DBTransactionNotFound);
        };
        db::Db::delete_transaction(&mut conn, transaction_id).await?;
        let (_, _, credit_account, credit_amount, debit_account, debit_amount) =
            transaction.into_parts();

        let credit_account =
            credit_account.reverse_update_balance(domain::FlowDirection::Incoming, credit_amount);
        let debit_account =
            debit_account.reverse_update_balance(domain::FlowDirection::Outgoing, debit_amount);

        db::Db::update_account(&mut conn, credit_account).await?;
        db::Db::update_account(&mut conn, debit_account).await?;

        if let Err(e) = conn.commit().await {
            return Err(DropTransactionError::DBCommitFailed(e));
        }

        Ok(())
    }

    pub async fn get_by_id(
        dbase: &SqlitePool,
        transaction_id: i64,
    ) -> Result<Self, ReadTransactionError> {
        let transaction_id = domain::TransactionID::try_from(transaction_id)?;
        let mut conn = dbase.acquire().await?;
        let Some(transaction) = db::Db::read_transaction(&mut conn, transaction_id).await? else {
            return Err(ReadTransactionError::DBTransactionNotFound);
        };
        Ok(transaction)
    }

    pub async fn list_all(dbase: &SqlitePool) -> Result<Vec<Self>, ListAllTransactionsError> {
        let mut conn = dbase.acquire().await?;
        let transactions = db::Db::read_all_transactions(&mut conn).await?;
        Ok(transactions)
    }

    pub async fn new(
        dbase: &SqlitePool,
        description: String,
        credit_account_id: i64,
        debit_account_id: i64,
        credit_amount: f64,
        debit_amount: Option<f64>,
    ) -> Result<Self, CreateNewTransactionError> {
        if credit_account_id == debit_account_id {
            return Err(CreateNewTransactionError::AccountsMustDiffer);
        }
        let credit_account_id = domain::AccountID::try_from(credit_account_id)
            .map_err(CreateNewTransactionError::InvalidCreditAccountID)?;
        let debit_account_id = domain::AccountID::try_from(debit_account_id)
            .map_err(CreateNewTransactionError::InvalidDebitAccountID)?;
        let mut conn = dbase.begin().await?;
        let Some(credit_account) = db::Db::read_account(&mut conn, credit_account_id)
            .await
            .map_err(CreateNewTransactionError::DBReadCreditAccountFailed)?
        else {
            return Err(CreateNewTransactionError::CreditAccountNotFound);
        };
        let Some(debit_account) = db::Db::read_account(&mut conn, debit_account_id)
            .await
            .map_err(CreateNewTransactionError::DBReadDebitAccountFailed)?
        else {
            return Err(CreateNewTransactionError::DebitAccountNotFound);
        };

        let actual_credit_amount = (credit_amount * 100.0) as i64;
        let actual_debit_amount = debit_amount.map_or(actual_credit_amount, |d| (d * 100.0) as i64);

        let credit_account =
            credit_account.update_balance(domain::FlowDirection::Outgoing, actual_credit_amount);
        let debit_account =
            debit_account.update_balance(domain::FlowDirection::Incoming, actual_debit_amount);

        if let Err(e) = db::Db::update_account(&mut conn, credit_account.clone()).await {
            return Err(CreateNewTransactionError::DBUpdateCreditAccountFailed(e));
        };
        if let Err(e) = db::Db::update_account(&mut conn, debit_account.clone()).await {
            return Err(CreateNewTransactionError::DBUpdateDebitAccountFailed(e));
        };

        let new_transaction = domain::NewTransaction::new(
            description,
            credit_account,
            actual_credit_amount,
            debit_account,
            actual_debit_amount,
        )?;

        let transaction = db::Db::create_transaction(&mut conn, new_transaction).await?;

        if let Err(e) = conn.commit().await {
            return Err(CreateNewTransactionError::DBSqlTransactionCommitFailed(e));
        };

        Ok(transaction)
    }
}
