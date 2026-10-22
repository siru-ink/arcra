use chrono::NaiveTime;

use crate::{db, domain};

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
    #[error(
        "Failed to load a currency exchange account for multi-currency transaction processing."
    )]
    ExchangeAccountNotFound(#[from] crate::services::ReadAccountError),
    #[error("Failed to read category from database.")]
    DBReadCategoryFailed(#[from] db::ReadCategoryError),
    #[error("No category id was provided, but a category id was needed for this transaction.")]
    CategoryIDNotProvided,
    #[error("Could not convert provided data into a CategoryID domain object.")]
    CreateCategoryIDFailed(#[from] domain::CategoryIDConstructorError),
    #[error("Failed to parse provided date into valid chrono::DateTime object.")]
    DateConversionFailed,
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

impl crate::services::Butler {
    pub async fn drop_transaction(&self, transaction_id: i64) -> Result<(), DropTransactionError> {
        let transaction_id = domain::TransactionID::try_from(transaction_id)?;
        let mut conn = self.dbase.begin().await?;
        let Some(transaction) = db::Db::read_transaction(&mut conn, transaction_id).await? else {
            return Err(DropTransactionError::DBTransactionNotFound);
        };
        db::Db::delete_transaction(&mut conn, transaction_id).await?;
        let (_, _, credit_account, credit_amount, debit_account, debit_amount) =
            transaction.into_parts();

        let credit_account =
            credit_account.reverse_update_balance(domain::FlowDirection::Outgoing, credit_amount);
        let debit_account =
            debit_account.reverse_update_balance(domain::FlowDirection::Incoming, debit_amount);

        db::Db::update_account(&mut conn, credit_account).await?;
        db::Db::update_account(&mut conn, debit_account).await?;

        if let Err(e) = conn.commit().await {
            return Err(DropTransactionError::DBCommitFailed(e));
        }

        Ok(())
    }

    pub async fn get_transaction_by_id(
        &self,
        transaction_id: i64,
    ) -> Result<domain::Transaction, ReadTransactionError> {
        let transaction_id = domain::TransactionID::try_from(transaction_id)?;
        let mut conn = self.dbase.acquire().await?;
        let Some(transaction) = db::Db::read_transaction(&mut conn, transaction_id).await? else {
            return Err(ReadTransactionError::DBTransactionNotFound);
        };
        Ok(transaction)
    }

    pub async fn list_all_transactions(
        &self,
    ) -> Result<Vec<domain::Transaction>, ListAllTransactionsError> {
        let mut conn = self.dbase.acquire().await?;
        let transactions = db::Db::read_all_transactions(&mut conn).await?;
        Ok(transactions)
    }

    pub async fn new_transaction(
        &self,
        description: String,
        date: Option<String>,
        credit_account_id: i64,
        debit_account_id: i64,
        credit_amount: f64,
        debit_amount: Option<f64>,
        category_id: Option<i64>,
    ) -> Result<domain::Transaction, CreateNewTransactionError> {
        if credit_account_id == debit_account_id {
            return Err(CreateNewTransactionError::AccountsMustDiffer);
        }

        let datetime = match date {
            Some(date) => chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                .map_err(|_| CreateNewTransactionError::DateConversionFailed)?
                .and_time(NaiveTime::MIN)
                .and_utc(),
            None => chrono::Utc::now(),
        };

        let credit_account_id = domain::AccountID::try_from(credit_account_id)
            .map_err(CreateNewTransactionError::InvalidCreditAccountID)?;
        let debit_account_id = domain::AccountID::try_from(debit_account_id)
            .map_err(CreateNewTransactionError::InvalidDebitAccountID)?;
        let mut conn = self.dbase.begin().await?;
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

        let null_category = db::Db::read_null_category(&mut conn).await?;
        let category = match (debit_account.account_type(), category_id) {
            (domain::AccountType::Expense, None) => {
                return Err(CreateNewTransactionError::CategoryIDNotProvided);
            }
            (domain::AccountType::Expense, Some(raw_category_id)) => {
                let category_id = domain::CategoryID::try_from(raw_category_id)?;
                db::Db::read_category(&mut conn, category_id).await?
            }
            _ => null_category.clone(),
        };

        let transaction: domain::Transaction;
        if credit_account.currency() != debit_account.currency() {
            let credit_exchange_account = self
                .get_currency_exchange_account(credit_account.currency().clone())
                .await?;
            let new_credit_transaction = domain::NewTransaction::new(
                format!("Mult. Curr. 1 Cr: {}", description),
                datetime,
                credit_account,
                actual_credit_amount,
                credit_exchange_account,
                actual_credit_amount,
                null_category,
            )?;

            let _credit_transaction =
                db::Db::create_transaction(&mut conn, new_credit_transaction).await?;

            let debit_exchange_account = self
                .get_currency_exchange_account(debit_account.currency().clone())
                .await?;
            let new_debit_transaction = domain::NewTransaction::new(
                format!("Mult. Curr. 2 Db: {}", description),
                datetime,
                debit_account,
                actual_debit_amount,
                debit_exchange_account,
                actual_debit_amount,
                category,
            )?;

            let debit_transaction =
                db::Db::create_transaction(&mut conn, new_debit_transaction).await?;

            transaction = debit_transaction
        } else {
            let new_transaction = domain::NewTransaction::new(
                description,
                datetime,
                credit_account,
                actual_credit_amount,
                debit_account,
                actual_debit_amount,
                category,
            )?;

            transaction = db::Db::create_transaction(&mut conn, new_transaction).await?;
        }

        if let Err(e) = conn.commit().await {
            return Err(CreateNewTransactionError::DBSqlTransactionCommitFailed(e));
        };

        Ok(transaction)
    }
}
