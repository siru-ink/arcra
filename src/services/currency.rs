use crate::{db, domain};

#[derive(Debug, thiserror::Error)]
pub enum CreateCurrencyError {
    #[error("The provided data could not be used to create a NewCurrency domain object.")]
    FailedNewCurrencyCheck(#[from] domain::CreateNewCurrencyError),
    #[error("Failed to obtain a connection to the database.")]
    DBConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to create a new currency record in the database.")]
    DBCreateCurrencyError(#[from] db::CreateCurrencyError),
    #[error("Failed to read the 'null' category record from database.")]
    DBReadNullCategoryFailed(#[from] db::ReadCategoryError),
    #[error("Failed to construct a NewAccount domain object from provided inputs.")]
    ConstructNewAccountObjectFailed(#[from] domain::CreateNewAccountError),
    #[error("Failed to commit/write the sql transaction to database.")]
    SqlTransactionCommitFailed(sqlx::Error),
    #[error("Failed to create a new account record in database.")]
    DBCreateExchangeAccountError(#[from] db::CreateAccountError),
}

#[derive(Debug, thiserror::Error)]
pub enum DropCurrencyError {
    #[error("Failed to convert provided id into a valid CurrencyID domain object.")]
    FailedCurrencyIDConversion(#[from] domain::CreateCurrencyIDError),
    #[error("Failed to obtain a connection to the database.")]
    DBConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to delete the reference Currency record in database.")]
    DBDropError(#[from] db::DeleteCurrencyError),
}

#[derive(Debug, thiserror::Error)]
pub enum ListAllCurrenciesError {
    #[error("Failed to retrieve data from database.")]
    DBReadError(#[from] db::ReadCurrencyError),
    #[error("Failed to retrieve a connection to the database.")]
    DBConnectionFailed(#[from] sqlx::Error),
}

impl crate::services::Butler {
    pub async fn create_currency(
        &self,
        name: String,
        descriptor: String,
        symbol: String,
    ) -> Result<domain::Currency, CreateCurrencyError> {
        let new_currency = domain::NewCurrency::new(name, descriptor, symbol)?;
        let mut conn = self.dbase.begin().await?;
        let currency = db::Db::create_currency(&mut conn, new_currency).await?;
        let new_exchange_account = domain::NewAccount::new(
            format!("{}_exchange", currency.name()),
            currency.clone(),
            domain::AccountType::CurrencyExchange,
        )?;
        let _exchange_account = db::Db::create_account(&mut conn, new_exchange_account).await?;
        if let Err(e) = conn.commit().await {
            return Err(CreateCurrencyError::SqlTransactionCommitFailed(e));
        }
        Ok(currency)
    }

    pub async fn drop_currency(&self, id: i64) -> Result<(), DropCurrencyError> {
        let currency_id = domain::CurrencyID::new(id)?;
        let mut conn = self.dbase.acquire().await?;
        db::Db::delete_currency(&mut conn, currency_id).await?;
        Ok(())
    }

    pub async fn list_all_currencies(
        &self,
    ) -> Result<Vec<domain::Currency>, ListAllCurrenciesError> {
        let mut conn = self.dbase.acquire().await?;
        let currencies = db::Db::read_all_currencies(&mut conn).await?;
        Ok(currencies)
    }
}
