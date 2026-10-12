#[derive(Debug, thiserror::Error)]
pub enum CreateSessionIDError {
    #[error("The provided id must be a positive integer.")]
    InvalidID,
}

pub struct SessionID(i64);

pub struct NewSession {}

pub struct Session {}

impl From<SessionID> for i64 {
    fn from(value: SessionID) -> Self {
        value.0
    }
}
impl SessionID {
    pub fn new(value: i64) -> Result<Self, CreateSessionIDError> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(CreateSessionIDError::InvalidID)
        }
    }
}
