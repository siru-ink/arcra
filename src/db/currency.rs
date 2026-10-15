use sqlx::SqliteConnection;

use crate::{db::Db, domain};

#[derive(Debug, thiserror::Error)]
pub enum CreateCurrencyError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
    #[error("Could not convert database row into domain object.")]
    HydrateError(#[from] domain::HydrateCurrencyError),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadCurrencyError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
    #[error("Could not convert database row into domain object.")]
    HydrateError(#[from] domain::HydrateCurrencyError),
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

impl TryFrom<CurrencyRow> for domain::Currency {
    type Error = domain::HydrateCurrencyError;

    fn try_from(value: CurrencyRow) -> Result<Self, Self::Error> {
        domain::Currency::hydrate(value.id, value.name, value.descriptor, value.symbol)
    }
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

        Ok(new_currency.try_into()?)
    }

    pub async fn read_currency(
        conn: &mut SqliteConnection,
        id: domain::CurrencyID,
    ) -> Result<Option<domain::Currency>, ReadCurrencyError> {
        let currency_row = sqlx::query_as!(
            CurrencyRow,
            r#"SELECT * FROM currencies WHERE id = ?"#,
            i64::from(id)
        )
        .fetch_optional(conn)
        .await?;

        Ok(currency_row.map(domain::Currency::try_from).transpose()?)
    }

    pub async fn update_currency(
        conn: &mut SqliteConnection,
        currency: domain::Currency,
    ) -> Result<(), UpdateCurrencyError> {
        let _ = sqlx::query!(
            "UPDATE currencies
             SET name = ?, descriptor = ?, symbol = ?
             WHERE id = ?",
            currency.name(),
            currency.descriptor(),
            currency.symbol(),
            currency.id()
        )
        .execute(conn)
        .await?;

        Ok(())
    }

    pub async fn delete_currency(
        conn: &mut SqliteConnection,
        id: domain::CurrencyID,
    ) -> Result<(), DeleteCurrencyError> {
        let _ = sqlx::query!("DELETE FROM currencies WHERE id = ?", i64::from(id))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn read_all_currencies(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<domain::Currency>, ReadCurrencyError> {
        let currency_rows = sqlx::query_as!(CurrencyRow, "SELECT * FROM currencies")
            .fetch_all(conn)
            .await?;

        let currencies: Vec<domain::Currency> = currency_rows
            .into_iter()
            .map(domain::Currency::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(currencies)
    }
}
