use crate::{
    AppState,
    ext::{Flash, Session},
    template::Publisher,
};
use axum::{
    Router,
    response::{IntoResponse, Response},
    routing,
};
use std::sync::Arc;

mod accounts;
mod analytics;
mod auth;
mod categories;
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
        .route("/categories/list", routing::get(categories::get_list))
        .route(
            "/categories/create",
            routing::get(categories::get_create).post(categories::post_create),
        )
        .route(
            "/categories/modify/{id}",
            routing::get(categories::get_modify),
        )
        .route("/categories/modify", routing::post(categories::post_modify))
        .route(
            "/categories/delete",
            routing::get(categories::get_delete).post(categories::post_delete),
        )
        .route(
            "/analytics/expense_by_category",
            routing::get(analytics::get_expense_by_category),
        )
        .route(
            "/analytics/check_accounting_equation",
            routing::get(analytics::get_check_accounting_equation),
        )
}

async fn get_index(publisher: Publisher, flash: Flash, _session: Session) -> Response {
    publisher.index_page(flash.message())
}

async fn get_healthcheck() -> Response {
    // TODO improve this
    "ok".into_response()
}
