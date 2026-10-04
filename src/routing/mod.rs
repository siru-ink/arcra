use crate::{AppState, flash::Flash, template::IndexPage};
use axum::{Router, extract::State, response::Response, routing};
use std::sync::Arc;

pub fn get() -> Router<Arc<AppState>> {
    Router::new().route("/", routing::get(get_index))
}

async fn get_index(State(state): State<Arc<AppState>>, flash: Flash) -> Response {
    IndexPage::show(&state.tera, flash.message())
}
