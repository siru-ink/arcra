use crate::{
    crumble::{Crumble, CrumbleKind},
    domain,
};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use sqlx::SqlitePool;

pub struct Session {
    crumble: Crumble,
    pool: SqlitePool,
}

impl Session {
    pub async fn valid(&self) -> bool {
        let raw_id = self.crumble.get(CrumbleKind::Session);
        let id = match raw_id.parse::<i64>() {
            Ok(id) => id,
            Err(_) => return false,
        };

        let Ok(session_id) = domain::SessionID::new(id) else {
            return false;
        };

        let Ok(mut db_connection) = self.pool.acquire().await else {
            return false;
        };

        session_id.exists_in_db(&mut db_connection).await
    }

    pub async fn set_new(&self) {
        let Ok(mut db_connection) = self.pool.acquire().await else {
            return;
        };

        let Ok(session) = domain::Session::create(&mut db_connection).await else {
            return;
        };

        self.crumble.set(CrumbleKind::Session, session.as_string());
    }

    pub async fn remove(&self) {
        // db::Session::vacuum(&self.pool).await;
        self.crumble.del(CrumbleKind::Session);
    }
}

impl FromRequestParts<std::sync::Arc<crate::AppState>> for Session {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &std::sync::Arc<crate::AppState>,
    ) -> Result<Self, Self::Rejection> {
        match Crumble::from_request_parts(parts, state).await {
            Ok(crumble) => Ok(Session {
                crumble,
                pool: state.pool.clone(),
            }),
            Err(e) => Err(e),
        }
    }
}
