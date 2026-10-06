use crate::{
    AppState,
    crumble::{Flash, Session},
    template::IndexPage,
};
use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Redirect, Response},
    routing,
};
use std::sync::Arc;

mod auth;

pub fn get() -> Router<Arc<AppState>> {
    Router::new().route("/", routing::get(get_index)).route(
        "/login",
        routing::get(auth::get_login).post(auth::post_login),
    )
}

async fn get_index(State(state): State<Arc<AppState>>, flash: Flash, session: Session) -> Response {
    match session.valid().await {
        true => IndexPage::show(&state.tera, flash.message()),
        false => Redirect::to("/login").into_response(),
    }
}
