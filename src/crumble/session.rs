use std::sync::Arc;

use crate::{
    AppState,
    crumble::{Crumble, CrumbleKind},
    db,
};
use axum::{
    extract::{FromRequestParts, State},
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

        db::Session::exists(&self.pool, id).await
    }

    pub async fn set_new(&self) {
        let session = db::Session::new(&self.pool).await;
        self.crumble
            .set(CrumbleKind::Session, session.id().to_string());
    }

    pub async fn remove(&self) {
        db::Session::vacuum(&self.pool).await;
        self.crumble.del(CrumbleKind::Session);
    }
}

impl FromRequestParts<Arc<AppState>> for Session {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let State(appstate): State<Arc<AppState>> = match State::from_request_parts(parts, state)
            .await
        {
            Ok(state) => state,
            Err(_) => {
                eprintln!(
                    "Failed to extract <AppState> from request for <Session::from_request_parts>."
                );
                return Err((StatusCode::INTERNAL_SERVER_ERROR, ""));
            }
        };

        match Crumble::from_request_parts(parts, state).await {
            Ok(crumble) => Ok(Session {
                crumble,
                pool: appstate.pool.clone(),
            }),
            Err(e) => Err(e),
        }
    }
}
