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

mod accounts;
mod auth;
mod currencies;
mod transactions;

pub fn get() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", routing::get(get_index))
        .route("/healthcheck", routing::get(get_healthcheck))
        .route(
            "/login",
            routing::get(auth::get_login).post(auth::post_login),
        )
        .route("/logout", routing::get(auth::get_logout))
        .route("/accounts/list", routing::get(accounts::get_accounts))
        .route(
            "/accounts/create",
            routing::get(accounts::get_create_account).post(accounts::post_create_account),
        )
        .route(
            "/accounts/delete",
            routing::get(accounts::get_delete_account).post(accounts::post_delete_account),
        )
        .route(
            "/accounts/modify",
            routing::get(accounts::get_modify_account).post(accounts::post_modify_account),
        )
        .route("/currencies/list", routing::get(currencies::get_currencies))
        .route(
            "/currencies/create",
            routing::get(currencies::get_create_currency).post(currencies::post_create_currency),
        )
        .route(
            "/currencies/delete",
            routing::get(currencies::get_delete_currency).post(currencies::post_delete_currency),
        )
        .route("/transactions/list", routing::get(transactions::get_list))
        .route(
            "/transactions/create",
            routing::get(transactions::get_create).post(transactions::post_create),
        )
        .route(
            "/transactions/delete",
            routing::get(transactions::get_delete).post(transactions::post_delete),
        )
}

async fn get_index(State(state): State<Arc<AppState>>, flash: Flash, session: Session) -> Response {
    match session.valid().await {
        true => IndexPage::show(&state.tera, flash.message()),
        false => Redirect::to("/login").into_response(),
    }
}

async fn get_healthcheck() -> Response {
    // TODO improve this
    "ok".into_response()
}
