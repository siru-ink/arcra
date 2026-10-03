use crate::AppState;
use axum::Router;
use std::sync::Arc;

pub fn get() -> Router<Arc<AppState>> {
    Router::new()
}
