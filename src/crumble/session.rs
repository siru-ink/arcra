use crate::{crumble::Crumble, domain, services};

pub struct Session {
    session: domain::Session,
}

#[derive(Debug, thiserror::Error)]
pub enum SessionExtractorError {
    #[error("Cookies could not be extracted from client request.")]
    CookieExtractorFailed,
    #[error("No valid/running session was found for this client.")]
    NoValidSession,
}

impl axum::response::IntoResponse for SessionExtractorError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::CookieExtractorFailed => {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to extract session from client request",
                )
                    .into_response();
            }
            Self::NoValidSession => return axum::response::Redirect::to("/login").into_response(),
        }
    }
}

impl axum::extract::FromRequestParts<std::sync::Arc<crate::AppState>> for Session {
    type Rejection = SessionExtractorError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &std::sync::Arc<crate::AppState>,
    ) -> Result<Self, Self::Rejection> {
        let crumble = Crumble::from_request_parts(parts, state)
            .await
            .map_err(|_| SessionExtractorError::CookieExtractorFailed)?;

        let Ok(butler) = services::Butler::from_request_parts(parts, state).await;

        let session = butler
            .get_session_from_crumble(crumble)
            .await
            .map_err(|_| SessionExtractorError::NoValidSession)?;

        Ok(Self { session })
    }
}
