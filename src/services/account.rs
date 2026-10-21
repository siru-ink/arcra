use crate::{db, domain};

#[derive(Debug, thiserror::Error)]
pub enum CreateAccountError {
    #[error("Could not convert currency_id to CurrencyID domain object.")]
    InvalidCurrencyID(#[from] domain::CreateCurrencyIDError),
    #[error("Failed to obtain a database connection from the database connection pool.")]
    DBConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to read currency information from database.")]
    ReadCurrencyFailed(#[from] db::ReadCurrencyError),
    #[error("No currency was found in database for given id.")]
    NoCurrencyFound,
    #[error("Could not convert account_type_id to AccountType domain object.")]
    InvalidAccountTypeID(#[from] domain::HydrateAccountTypeError),
    #[error("Could not convert given variables into a new NewAccount domain object.")]
    InvalidNewAccount(#[from] domain::CreateNewAccountError),
    #[error("Failed to create a new account in the database.")]
    CreateDBAccountFailed(#[from] db::CreateAccountError),
    #[error("The provided category id value is not valid for the selected account type.")]
    InvalidCategoryIDAccountTypeMatchup,
    #[error("Could not convert category_id to CategoryID domain object.")]
    InvalidCategoryID(#[from] domain::CategoryIDConstructorError),
    #[error("Failed to read category information from database.")]
    DBReadCategoryFailed(#[from] db::ReadCategoryError),
}

#[derive(Debug, thiserror::Error)]
pub enum DropAccountError {
    #[error("Could not convert i64 id into AccountID domain object.")]
    InvalidID(#[from] domain::CreateAccountIDError),
    #[error("Failed to obtain a database connection.")]
    DBConnectionIssue(#[from] sqlx::Error),
    #[error("Failed to delete account in database.")]
    CouldNotDeleteAccount(#[from] db::DeleteAccountError),
}

#[derive(Debug, thiserror::Error)]
pub enum ListAllAccountsError {
    #[error("Failed to obtain a database connection.")]
    DBConnectionIssue(#[from] sqlx::Error),
    #[error("Failed to read all accounts from database.")]
    RBReadFailure(#[from] db::ReadAccountError),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadAccountError {
    #[error("Could not convert i64 id into AccountID domain object.")]
    InvalidID(#[from] domain::CreateAccountIDError),
    #[error("Failed to obtain a database connection.")]
    DBConnectionIssue(#[from] sqlx::Error),
    #[error("Could not read account by id in database.")]
    DBReadFailure(#[from] db::ReadAccountError),
    #[error("No account was found by this id in the database.")]
    NoAccountFound,
}

#[derive(Debug, thiserror::Error)]
pub enum RenameAccountError {
    #[error("Could not convert given id into AccountID domain object.")]
    InvalidAccountID(#[from] domain::CreateAccountIDError),
    #[error("Failed to obtain a database connection.")]
    DBConnectionIssue(#[from] sqlx::Error),
    #[error("Could not read account by id from database.")]
    DBReadFailed(#[from] db::ReadAccountError),
    #[error("No account was found by this id in the database.")]
    NoAccountFound,
    #[error("A new name must be provided.")]
    MissingNewName,
}

impl crate::services::Butler {
    pub async fn drop_multiple_accounts(&self, ids: &Vec<i64>) -> Result<(), DropAccountError> {
        let account_ids: Vec<domain::AccountID> = ids
            .into_iter()
            .map(domain::AccountID::try_from)
            .collect::<Result<_, _>>()?;

        let mut conn = self.dbase.acquire().await?;

        for id in account_ids {
            db::Db::delete_account(&mut conn, id).await?;
        }

        Ok(())
    }

    pub async fn get_account_by_id(
        &self,
        account_id: i64,
    ) -> Result<domain::Account, ReadAccountError> {
        let account_id = domain::AccountID::try_from(&account_id)?;
        let mut conn = self.dbase.acquire().await?;
        let account = db::Db::read_account(&mut conn, account_id).await?;
        match account {
            Some(account) => Ok(account),
            None => Err(ReadAccountError::NoAccountFound),
        }
    }

    pub async fn get_currency_exchange_account(
        &self,
        currency: domain::Currency,
    ) -> Result<domain::Account, ReadAccountError> {
        let mut conn = self.dbase.acquire().await?;
        let exchange_accounts = db::Db::read_all_exchange_accounts_only(&mut conn).await?;
        for account in exchange_accounts {
            if *account.currency() == currency {
                return Ok(account);
            }
        }
        Err(ReadAccountError::NoAccountFound)
    }

    pub async fn list_all_accounts(&self) -> Result<Vec<domain::Account>, ListAllAccountsError> {
        let mut conn = self.dbase.acquire().await?;
        let accounts = db::Db::read_all_non_exchange_accounts(&mut conn).await?;
        Ok(accounts)
    }

    pub async fn new_account(
        &self,
        name: String,
        currency_id: i64,
        account_type_id: &str,
        category_id: i64,
    ) -> Result<domain::Account, CreateAccountError> {
        let currency_id = domain::CurrencyID::try_from(currency_id)?;
        let mut conn = self.dbase.acquire().await?;
        let Some(currency) = db::Db::read_currency(&mut conn, currency_id).await? else {
            return Err(CreateAccountError::NoCurrencyFound);
        };
        let account_type = domain::AccountType::try_from(account_type_id)?;
        let category_id = match account_type {
            domain::AccountType::Expense if category_id > 1 => category_id,
            domain::AccountType::Expense => {
                return Err(CreateAccountError::InvalidCategoryIDAccountTypeMatchup);
            }
            _ => 1,
        };
        let category_id = domain::CategoryID::try_from(category_id)?;
        let category = db::Db::read_category(&mut conn, category_id).await?;
        let new_account = domain::NewAccount::new(name, currency, account_type, category)?;
        let account = db::Db::create_account(&mut conn, new_account).await?;
        Ok(account)
    }

    pub async fn rename_account(
        &self,
        account_id: i64,
        new_name: String,
    ) -> Result<domain::Account, RenameAccountError> {
        if new_name.is_empty() {
            // this should probably be checked inside of set_name fn instead,
            // feels like a waste to go through all that effort though to only
            // notice later on
            return Err(RenameAccountError::MissingNewName);
        }
        let account_id = domain::AccountID::try_from(&account_id)?;
        let mut conn = self.dbase.acquire().await?;
        let Some(account) = db::Db::read_account(&mut conn, account_id).await? else {
            return Err(RenameAccountError::NoAccountFound);
        };
        let account = account.set_name(new_name);
        Ok(account)
    }
}
