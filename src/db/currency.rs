use sqlx::SqliteConnection;

use crate::{db::Db, domain};

#[derive(Debug, thiserror::Error)]
pub enum CreateCurrencyError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadCurrencyError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateCurrencyError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteCurrencyError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
}

struct CurrencyRow {
    id: i64,
    name: String,
    descriptor: String,
    symbol: String,
}

impl Db {
    pub async fn create_currency(
        conn: &mut SqliteConnection,
        new_currency: domain::NewCurrency,
    ) -> Result<domain::Currency, CreateCurrencyError> {
        let new_id = sqlx::query!(
            r#"INSERT INTO currencies (name, descriptor, symbol)
               VALUES (?,?,?)
               RETURNING id"#,
            new_currency.name(),
            new_currency.descriptor(),
            new_currency.symbol()
        )
        .fetch_one(&mut *conn)
        .await?
        .id;

        let new_currency = sqlx::query_as!(
            CurrencyRow,
            r#"SELECT * FROM currencies WHERE id = ?"#,
            new_id
        )
        .fetch_one(conn)
        .await?;

        Ok(domain::Currency::hydrate(
            new_currency.id,
            new_currency.name,
            new_currency.descriptor,
            new_currency.symbol,
        ))
    }

    pub async fn read_currency(
        conn: &mut SqliteConnection,
    ) -> Result<Option<domain::Currency>, ReadCurrencyError> {
        todo!()
    }

    pub async fn update_currency(conn: &mut SqliteConnection) -> Result<(), UpdateCurrencyError> {
        todo!()
    }

    pub async fn delete_currency(conn: &mut SqliteConnection) -> Result<(), DeleteCurrencyError> {
        todo!()
    }
}
