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
}

impl Db {
    pub async fn create_account(
        conn: &mut SqliteConnection,
        account: domain::NewAccount,
    ) -> Result<domain::Account, CreateAccountError> {
        let new_id = sqlx::query!(
            r#"INSERT INTO accounts (name, currency, account_type)
               VALUES (?,?,?)
               RETURNING id as "id!""#,
            account.name(),
            account.currency().id(),
            account.account_type() as i64,
        )
        .fetch_one(&mut *conn)
        .await?
        .id;

        let new_account = sqlx::query_as!(
            AccountRow,
            r#"SELECT
                 a.id as id,
                 a.name as name,
                 c.id as currency_id,
                 c.name as currency_name,
                 c.descriptor as currency_descriptor,
                 c.symbol as currency_symbol,
                 a.balance as balance,
                 a.account_type as account_type
               FROM accounts a
               JOIN currencies c on a.currency = c.id
               WHERE a.id = ?"#,
            new_id
        )
        .fetch_one(conn)
        .await?;

        Ok(domain::Account::hydrate(
            new_account.id,
            new_account.name,
            new_account.currency_id,
            new_account.currency_name,
            new_account.currency_descriptor,
            new_account.currency_symbol,
            new_account.balance,
            new_account.account_type,
        )?)
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
                 c.id as currency_id,
                 c.name as currency_name,
                 c.descriptor as currency_descriptor,
                 c.symbol as currency_symbol,
                 a.balance as balance,
                 a.account_type as account_type
             FROM accounts a
             JOIN currencies c on a.currency = c.id
             WHERE a.id = ?",
            id.0
        )
        .fetch_optional(conn)
        .await?;

        Ok(account_row.map(domain::Account::try_from).transpose()?)
    }

    pub async fn update_account(
        conn: &mut SqliteConnection,
        account: domain::Account,
    ) -> Result<(), UpdateAccountError> {
        todo!()
    }

    pub async fn delete_account(
        conn: &mut SqliteConnection,
        account: domain::Account,
    ) -> Result<(), DeleteAccountError> {
        todo!()
    }
}
