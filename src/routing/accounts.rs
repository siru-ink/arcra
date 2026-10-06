use crate::{
    AppState,
    crumble::Session,
    db::{Account, AccountType, Currency},
    template::{AccountCreatePage, AccountListPage},
};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
};
use std::sync::Arc;

pub async fn get_accounts(State(state): State<Arc<AppState>>, session: Session) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    };

    let accounts = Account::list_all(&state.pool).await;

    AccountListPage::show(&state.tera, accounts)
}

pub async fn get_create_account(State(state): State<Arc<AppState>>, session: Session) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    };

    let currencies = Currency::list_all(&state.pool).await;
    let account_types = AccountType::list_all(&state.pool).await;

    AccountCreatePage::show(&state.tera, currencies, account_types)
}

pub async fn post_create_account() -> Response {
    todo!()
}

pub async fn get_delete_account() -> Response {
    todo!()
}

pub async fn post_delete_account() -> Response {
    todo!()
}
