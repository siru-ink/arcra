pub struct Service {
    dbase: sqlx::SqlitePool,
}

impl axum::extract::FromRequestParts<std::sync::Arc<crate::AppState>> for Service {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        _parts: &mut axum::http::request::Parts,
        state: &std::sync::Arc<crate::AppState>,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self {
            dbase: state.pool.clone(),
        })
    }
}
