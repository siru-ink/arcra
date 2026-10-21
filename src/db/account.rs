use crate::{db::Db, domain};
use sqlx::SqliteConnection;

#[derive(Debug, thiserror::Error)]
pub enum CreateAccountError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
    #[error("Could not convert database row into Account domain object.")]
    HydrateError(#[from] domain::HydrateAccountError),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadAccountError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
    #[error("Could not convert database row into Account domain object.")]
    HydrateError(#[from] domain::HydrateAccountError),
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateAccountError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteAccountError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
}

impl TryFrom<AccountRow> for domain::Account {
    type Error = domain::HydrateAccountError;
    fn try_from(value: AccountRow) -> Result<Self, Self::Error> {
        domain::Account::hydrate(
            value.id,
            value.name,
            value.currency_id,
            value.currency_name,
            value.currency_descriptor,
            value.currency_symbol,
            value.balance,
            value.account_type,
            value.category_id,
            value.category_name,
            value.category_description,
        )
    }
}

struct AccountRow {
    id: i64,
    name: String,
    currency_id: i64,
    currency_name: String,
    currency_descriptor: String,
    currency_symbol: String,
    balance: i64,
    account_type: i64,
    category_id: i64,
    category_name: String,
    category_description: String,
}

impl Db {
    pub async fn create_account(
        conn: &mut SqliteConnection,
        account: domain::NewAccount,
    ) -> Result<domain::Account, CreateAccountError> {
        let new_id = sqlx::query!(
            r#"INSERT INTO accounts (name, currency, balance, account_type, category)
               VALUES (?, ?, 0, ?, ?)
               RETURNING id"#,
            account.name(),
            account.currency().id(),
            account.account_type() as i64,
            account.category().id()
        )
        .fetch_one(&mut *conn)
        .await?
        .id;

        let new_account = sqlx::query_as!(
            AccountRow,
            r#"SELECT
                 a.id as id,
                 a.name as name,
                 cu.id as currency_id,
                 cu.name as currency_name,
                 cu.descriptor as currency_descriptor,
                 cu.symbol as currency_symbol,
                 a.balance as balance,
                 a.account_type as account_type,
                 ca.id as category_id,
                 ca.name as category_name,
                 ca.description as category_description
               FROM accounts a
               JOIN currencies cu on a.currency = cu.id
               JOIN categories ca on a.category = ca.id
               WHERE a.id = ?"#,
            new_id
        )
        .fetch_one(conn)
        .await?;

        Ok(new_account.try_into()?)
    }

    pub async fn read_account(
        conn: &mut SqliteConnection,
        id: domain::AccountID,
    ) -> Result<Option<domain::Account>, ReadAccountError> {
        let account_row = sqlx::query_as!(
            AccountRow,
            "SELECT
                 a.id as id,
                 a.name as name,
                 cu.id as currency_id,
                 cu.name as currency_name,
                 cu.descriptor as currency_descriptor,
                 cu.symbol as currency_symbol,
                 a.balance as balance,
                 a.account_type as account_type,
                 ca.id as category_id,
                 ca.name as category_name,
                 ca.description as category_description
             FROM accounts a
             JOIN currencies cu on a.currency = cu.id
             JOIN categories ca on a.category = ca.id
             WHERE a.id = ?",
            i64::from(id)
        )
        .fetch_optional(conn)
        .await?;

        Ok(account_row.map(domain::Account::try_from).transpose()?)
    }

    pub async fn update_account(
        conn: &mut SqliteConnection,
        account: domain::Account,
    ) -> Result<(), UpdateAccountError> {
        let _ = sqlx::query!(
            "UPDATE accounts
            SET name = ?, currency = ?, balance = ?, account_type = ?
            WHERE id = ?",
            account.name(),
            account.currency_id(),
            account.balance(),
            account.account_type_id(),
            account.id()
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn delete_account(
        conn: &mut SqliteConnection,
        id: domain::AccountID,
    ) -> Result<(), DeleteAccountError> {
        let _ = sqlx::query!("DELETE FROM accounts WHERE id = ?", i64::from(id))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn read_all_non_exchange_accounts(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<domain::Account>, ReadAccountError> {
        let account_rows = sqlx::query_as!(
            AccountRow,
            "SELECT
                 a.id as id,
                 a.name as name,
                 cu.id as currency_id,
                 cu.name as currency_name,
                 cu.descriptor as currency_descriptor,
                 cu.symbol as currency_symbol,
                 a.balance as balance,
                 a.account_type as account_type,
                 ca.id as category_id,
                 ca.name as category_name,
                 ca.description as category_description
             FROM accounts a
             JOIN currencies cu on a.currency = cu.id
             JOIN categories ca on a.category = ca.id
             WHERE a.account_type != ?",
            domain::AccountType::CurrencyExchange.id(),
        )
        .fetch_all(conn)
        .await?;

        let accounts: Vec<domain::Account> = account_rows
            .into_iter()
            .map(domain::Account::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(accounts)
    }

    pub async fn read_all_exchange_accounts_only(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<domain::Account>, ReadAccountError> {
        let account_rows = sqlx::query_as!(
            AccountRow,
            "SELECT
                 a.id as id,
                 a.name as name,
                 cu.id as currency_id,
                 cu.name as currency_name,
                 cu.descriptor as currency_descriptor,
                 cu.symbol as currency_symbol,
                 a.balance as balance,
                 a.account_type as account_type,
                 ca.id as category_id,
                 ca.name as category_name,
                 ca.description as category_description
             FROM accounts a
             JOIN currencies cu on a.currency = cu.id
             JOIN categories ca on a.category = ca.id
             WHERE a.account_type = ?",
            domain::AccountType::CurrencyExchange.id(),
        )
        .fetch_all(conn)
        .await?;

        let accounts: Vec<domain::Account> = account_rows
            .into_iter()
            .map(domain::Account::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(accounts)
    }
}
