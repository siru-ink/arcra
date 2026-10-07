use crate::{AppState, crumble::Flash, db::Account, template::TransactionCreatePage};
use axum::{extract::State, response::Response};
use std::sync::Arc;

pub async fn get_create(State(state): State<Arc<AppState>>, flash: Flash) -> Response {
    let accounts = Account::list_all(&state.pool).await;
    TransactionCreatePage::show(&state.tera, accounts, flash.message())
}
