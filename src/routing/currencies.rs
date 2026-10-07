use std::sync::Arc;

use axum::{extract::State, response::Response};

use crate::{AppState, crumble::Flash, db::Currency, template::CurrencyListPage};

pub async fn get_currencies(State(state): State<Arc<AppState>>, flash: Flash) -> Response {
    let currencies = Currency::list_all(&state.pool).await;
    CurrencyListPage::show(&state.tera, currencies, flash.message())
}

pub async fn get_create_currency() -> Response {
    todo!()
}

pub async fn post_create_currency() -> Response {
    todo!()
}

pub async fn get_delete_currency() -> Response {
    todo!()
}

pub async fn post_delete_currency() -> Response {
    todo!()
}
