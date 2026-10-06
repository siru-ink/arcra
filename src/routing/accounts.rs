use crate::{
    AppState,
    crumble::{Flash, Session},
    db::{Account, AccountType, Currency},
    template::{AccountCreatePage, AccountDeletePage, AccountListPage, AccountModifyPage},
};
use axum::{
    Form,
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};

pub async fn get_accounts(
    State(state): State<Arc<AppState>>,
    session: Session,
    flash: Flash,
) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    };

    let accounts = Account::list_all(&state.pool).await;

    AccountListPage::show(&state.tera, accounts, flash.message())
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

pub async fn get_modify_account(
    State(state): State<Arc<AppState>>,
    Query(param): Query<HashMap<String, String>>,
    flash: Flash,
) -> Response {
    let account_id = match param
        .get("account_id")
        .and_then(|rawid| rawid.parse::<i64>().ok())
    {
        Some(id) => id,
        None => {
            flash.set("<account_id> parameter was missing from GET request. Please select one of the below accounts.".to_string());
            return Redirect::to("/accounts/list").into_response();
        }
    };

    let account = match Account::get(&state.pool, account_id).await {
        Some(account) => account,
        None => {
            flash.set("No account could be found with the provided id. Please select one of the accounts below.".to_string());
            return Redirect::to("/accounts/list").into_response();
        }
    };

    AccountModifyPage::show(&state.tera, account)
}

#[derive(Deserialize)]
pub struct AccountModifyForm {
    account_id: i64,
    name: String,
}

pub async fn post_modify_account(
    State(state): State<Arc<AppState>>,
    flash: Flash,
    form: Form<AccountModifyForm>,
) -> Response {
    let account = match Account::get(&state.pool, form.account_id).await {
        Some(account) => account,
        None => return Redirect::to("/accounts/list").into_response(),
    };

    account.rename(&state.pool, &form.name).await;

    flash.set("Account renamed successfully.".to_string());
    Redirect::to("/accounts/list").into_response()
}
