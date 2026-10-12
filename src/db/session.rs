use crate::{db::Db, domain};
use sqlx::SqliteConnection;

#[derive(Debug, thiserror::Error)]
pub enum CreateSessionError {
    #[error("Failure in sqlx database connection.")]
    SqlError(#[from] sqlx::Error),
    #[error("Could not convert database row into domain object.")]
    HydrationError,
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

impl Db {
    pub async fn create(
        conn: &mut SqliteConnection,
        new_session: domain::NewSession,
    ) -> Result<domain::Session, CreateSessionError> {
        todo!()
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
