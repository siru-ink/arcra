use crate::{db, domain};

#[derive(Debug, thiserror::Error)]
pub enum CreateCurrencyError {
    #[error("The provided data could not be used to create a NewCurrency domain object.")]
    FailedNewCurrencyCheck(#[from] domain::CreateNewCurrencyError),
    #[error("Failed to obtain a connection to the database.")]
    DBConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to create a new currency record in the database.")]
    DBCreateError(#[from] db::CreateCurrencyError),
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
        self,
        name: String,
        descriptor: String,
        symbol: String,
    ) -> Result<domain::Currency, CreateCurrencyError> {
        let new_currency = domain::NewCurrency::new(name, descriptor, symbol)?;
        let mut conn = self.dbase.acquire().await?;
        let currency = db::Db::create_currency(&mut conn, new_currency).await?;
        Ok(currency)
    }

    pub async fn drop_currency(self, id: i64) -> Result<(), DropCurrencyError> {
        let currency_id = domain::CurrencyID::new(id)?;
        let mut conn = self.dbase.acquire().await?;
        db::Db::delete_currency(&mut conn, currency_id).await?;
        Ok(())
    }

    pub async fn list_all_currencies(
        self,
    ) -> Result<Vec<domain::Currency>, ListAllCurrenciesError> {
        let mut conn = self.dbase.acquire().await?;
        let currencies = db::Db::read_all_currencies(&mut conn).await?;
        Ok(currencies)
    }
}
