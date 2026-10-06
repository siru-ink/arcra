use crate::crumble::{Crumble, CrumbleKind};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

pub struct Flash {
    crumble: Crumble,
}

impl Flash {
    pub fn message(self) -> Option<String> {
        let message = self.crumble.get(CrumbleKind::Flash);

        self.crumble.del(CrumbleKind::Flash);

        if message.is_empty() {
            None
        } else {
            Some(message)
        }
    }
    pub fn set(self, data: String) {
        if !data.is_empty() {
            self.crumble.set(CrumbleKind::Flash, data);
        }
    }
}

impl<S> FromRequestParts<S> for Flash
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Crumble::from_request_parts(parts, state).await {
            Ok(crumble) => Ok(Flash { crumble }),
            Err(e) => Err(e),
        }
    }
}
