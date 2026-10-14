use crate::{db::Db, domain};
use sqlx::SqliteConnection;

#[derive(Debug, thiserror::Error)]
pub enum CreateSessionError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
    #[error("Could not convert database row into domain object.")]
    HydrateError(#[from] domain::HydrateSessionError),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadSessionError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
    #[error("Could not convert database row into domain object.")]
    HydrationError,
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteSessionError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
}

pub struct SessionRow {
    id: i64,
    valid_until: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<SessionRow> for domain::Session {
    type Error = domain::HydrateSessionError;

    fn try_from(value: SessionRow) -> Result<Self, Self::Error> {
        Self::hydrate(value.id, value.valid_until)
    }
}

impl Db {
    pub async fn create(
        conn: &mut SqliteConnection,
        new_session: domain::NewSession,
    ) -> Result<domain::Session, CreateSessionError> {
        let new_id = sqlx::query!(
            "INSERT INTO sessions (valid_until)
             VALUES (?)
             RETURNING id",
            new_session.valid_until()
        )
        .fetch_one(&mut *conn)
        .await?
        .id;

        let new_session = sqlx::query_as!(
            SessionRow,
            r#"SELECT id, valid_until as "valid_until: _" FROM sessions WHERE id = ?"#,
            new_id
        )
        .fetch_one(conn)
        .await?;

        Ok(domain::Session::try_from(new_session)?)
    }

    pub async fn read(
        conn: &mut SqliteConnection,
        id: domain::SessionID,
    ) -> Result<(), ReadSessionError> {
        todo!()
    }

    pub async fn delete(
        conn: &mut SqliteConnection,
        id: domain::SessionID,
    ) -> Result<(), DeleteSessionError> {
        todo!()
    }
}
