use crate::{db, domain, services};
use sqlx::SqliteConnection;

#[derive(Debug, thiserror::Error)]
pub enum GetSessionFromCrumbleError {
    #[error("Cookie session data was empty.")]
    SessionCookieEmpty,
    #[error("Failed to parse cookie data into a number.")]
    CookieParseFailed(#[from] std::num::ParseIntError),
    #[error("Failed to convert id into a valid SessionID domain object.")]
    SessionIDConversionFailed(#[from] domain::CreateSessionIDError),
    #[error("Failed to connect to the database.")]
    SqlConnectionFailed(#[from] sqlx::Error),
    #[error("Session data could not be read from database.")]
    DBReasSessionFailed(#[from] db::ReadSessionError),
    #[error("No valid session could be found.")]
    NoSessionFound,
}

#[derive(Debug, thiserror::Error)]
pub enum RemoveSessionError {}

#[derive(Debug, thiserror::Error)]
pub enum SaveNewSessionError {
    #[error("Failed to connect to the database.")]
    SqlConnectionFailed(#[from] sqlx::Error),
    #[error("Failed to save new session in database.")]
    DBCreateSessionFailed(#[from] db::CreateSessionError),
}

impl domain::SessionID {
    pub async fn exists_in_db(self, conn: &mut SqliteConnection) -> bool {
        match db::Db::read_session(conn, self).await {
            Ok(Some(_)) => true,
            _ => false,
        }
    }
}

impl services::Butler {
    pub async fn get_session_from_crumble(
        &self,
        crumble: crate::ext::crumble::Crumble,
    ) -> Result<domain::Session, GetSessionFromCrumbleError> {
        let raw_id = crumble.get(crate::ext::crumble::CrumbleKind::Session);
        if raw_id.is_empty() {
            return Err(GetSessionFromCrumbleError::SessionCookieEmpty);
        }

        let id = raw_id.parse::<i64>()?;
        let session_id = domain::SessionID::try_from(id)?;
        let mut conn = self.dbase.acquire().await?;
        let Some(session) = db::Db::read_session(&mut conn, session_id).await? else {
            return Err(GetSessionFromCrumbleError::NoSessionFound);
        };
        Ok(session)
    }

    pub fn remove_session_from_client(&self) -> Result<(), RemoveSessionError> {
        todo!()
    }

    pub async fn save_new_session(&self) -> Result<(), SaveNewSessionError> {
        let new_session = domain::NewSession::new();
        let mut conn = self.dbase.acquire().await?;
        let _ = db::Db::create_session(&mut conn, new_session).await?;
        Ok(())
    }
}
