use crate::{AppState, template::IndexPage};
use axum::{Router, extract::State, response::Response, routing};
use std::sync::Arc;

pub fn get() -> Router<Arc<AppState>> {
    Router::new().route("/", routing::get(get_index))
}

async fn get_index(State(state): State<Arc<AppState>>) -> Response {
    IndexPage::show(&state.tera)
}
