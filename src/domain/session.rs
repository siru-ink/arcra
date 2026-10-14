#[derive(Debug, thiserror::Error)]
pub enum CreateSessionIDError {
    #[error("The provided id must be a positive integer.")]
    InvalidID,
}

#[derive(Debug, thiserror::Error)]
pub enum HydrateSessionError {
    #[error("The provided id must be a positive integer.")]
    InvalidID,
    #[error("The provided valid_until value must be in the future.")]
    InvalidValidUntil,
}

pub struct SessionID(i64);

pub struct NewSession {
    valid_until: chrono::DateTime<chrono::Utc>,
}

pub struct Session {
    id: i64,
    valid_until: chrono::DateTime<chrono::Utc>,
}

impl From<SessionID> for i64 {
    fn from(value: SessionID) -> Self {
        value.0
    }
}

impl From<Session> for SessionID {
    fn from(value: Session) -> Self {
        SessionID(value.id)
    }
}

impl NewSession {
    pub fn new() -> Self {
        NewSession {
            valid_until: chrono::Utc::now() + chrono::TimeDelta::weeks(1),
        }
    }

    pub fn valid_until(&self) -> chrono::DateTime<chrono::Utc> {
        self.valid_until
    }
}

impl Session {
    pub fn hydrate(
        id: i64,
        valid_until: chrono::DateTime<chrono::Utc>,
    ) -> Result<Self, HydrateSessionError> {
        if id <= 0 {
            return Err(HydrateSessionError::InvalidID);
        } else if valid_until <= chrono::Utc::now() {
            return Err(HydrateSessionError::InvalidValidUntil);
        }

        Ok(Self { id, valid_until })
    }
}
