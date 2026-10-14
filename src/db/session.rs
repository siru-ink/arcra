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
    HydrateError(#[from] domain::HydrateSessionError),
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
    pub async fn create_session(
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

    pub async fn read_session(
        conn: &mut SqliteConnection,
        id: domain::SessionID,
    ) -> Result<Option<domain::Session>, ReadSessionError> {
        let session_row = sqlx::query_as!(
            SessionRow,
            r#"SELECT id, valid_until as "valid_until: _" FROM sessions WHERE id = ?"#,
            i64::from(id)
        )
        .fetch_optional(conn)
        .await?;

        Ok(session_row.map(domain::Session::try_from).transpose()?)
    }

    pub async fn delete_session(
        conn: &mut SqliteConnection,
        id: domain::SessionID,
    ) -> Result<(), DeleteSessionError> {
        let _ = sqlx::query!("DELETE FROM sessions WHERE id = ?", i64::from(id))
            .execute(conn)
            .await?;

        Ok(())
    }
}
