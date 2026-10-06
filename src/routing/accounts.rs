use crate::{
    AppState,
    crumble::{Flash, Session},
    db::{Account, AccountType, Currency},
    template::{AccountCreatePage, AccountDeletePage, AccountListPage},
};
use axum::{
    Form,
    extract::State,
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;
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

#[derive(Deserialize)]
pub struct AccountCreateForm {
    name: String,
    currency: i64,
    account_type: i64,
}

pub async fn post_create_account(
    State(state): State<Arc<AppState>>,
    flash: Flash,
    form: Form<AccountCreateForm>,
) -> Response {
    Account::new(&state.pool, &form.name, form.currency, form.account_type).await;
    flash.set("New account created.".to_string());
    Redirect::to("/").into_response()
}

pub async fn get_delete_account(State(state): State<Arc<AppState>>) -> Response {
    let accounts = Account::list_all(&state.pool).await;
    AccountDeletePage::show(&state.tera, accounts)
}

pub async fn post_delete_account() -> Response {
    todo!()
}

pub async fn get_modify_account() -> Response {
    todo!()
}

pub async fn post_modify_account() -> Response {
    todo!()
}
