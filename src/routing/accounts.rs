use crate::{
    AppState,
    crumble::{Flash, Session},
    domain,
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

    let Ok(accounts) = domain::Account::list_all(&state.pool).await else {
        flash.set("Failed to load all accounts.".to_string());
        return Redirect::to("/").into_response();
    };

    AccountListPage::show(&state.tera, accounts, flash.message())
}

pub async fn get_create_account(
    State(state): State<Arc<AppState>>,
    flash: Flash,
    session: Session,
) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    };

    let Ok(currencies) = domain::Currency::list_all(&state.pool).await else {
        flash.set("Failed to load all currencies.".to_string());
        return Redirect::to("/").into_response();
    };

    let account_types = domain::AccountType::list_all();

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
    if let Err(_) = domain::Account::new(
        &state.pool,
        form.name.clone(),
        form.currency,
        form.account_type,
    )
    .await
    {
        flash.set("Failed to create new account.".to_string());
        return Redirect::to("/").into_response();
    }
    flash.set("New account created.".to_string());
    Redirect::to("/").into_response()
}

pub async fn get_delete_account(State(state): State<Arc<AppState>>, flash: Flash) -> Response {
    let Ok(accounts) = domain::Account::list_all(&state.pool).await else {
        flash.set("Failed to load all accounts.".to_string());
        return Redirect::to("/").into_response();
    };
    AccountDeletePage::show(&state.tera, accounts)
}

#[derive(Deserialize)]
pub struct AccountDeleteForm {
    #[serde(default)]
    ids: Vec<i64>,
}

pub async fn post_delete_account(
    State(state): State<Arc<AppState>>,
    flash: Flash,
    form: axum_extra::extract::Form<AccountDeleteForm>,
) -> Response {
    if form.ids.len() == 0 {
        return Redirect::to("/accounts/delete").into_response();
    }

    if let Err(_) = domain::Account::drop_multiple(&state.pool, &form.ids).await {
        flash.set("Failed to delete some account(s).".to_string());
    } else {
        flash.set("Account(s) were deleted.".to_string());
    };
    Redirect::to("/accounts/list").into_response()
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

    let Ok(account) = domain::Account::get_by_id(&state.pool, account_id).await else {
        flash.set("No account could be retrieved for the given id.".to_string());
        return Redirect::to("/accounts/list").into_response();
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
    match domain::Account::rename(&state.pool, form.account_id, form.name.clone()).await {
        Ok(_) => flash.set("Account renamed successfully.".to_string()),
        Err(_) => flash.set("Could not rename account.".to_string()),
    };
    Redirect::to("/accounts/list").into_response()
}
