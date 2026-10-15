use crate::{db, domain};
use sqlx::SqliteConnection;

#[derive(Debug, thiserror::Error)]
pub enum CreateSessionServiceError {
    #[error("Database failed to be able to create a new session.")]
    DbCreateSessionError(#[from] db::CreateSessionError),
}

impl domain::SessionID {
    pub async fn exists_in_db(self, conn: &mut SqliteConnection) -> bool {
        match db::Db::read_session(conn, self).await {
            Ok(Some(_)) => true,
            _ => false,
        }
    }
}

impl domain::Session {
    pub async fn create(conn: &mut SqliteConnection) -> Result<Self, CreateSessionServiceError> {
        let new_session = domain::NewSession::new();
        let session = db::Db::create_session(conn, new_session).await?;
        Ok(session)
    }

    pub fn as_string(&self) -> String {
        self.id().to_string()
    }
}
