use crate::{
    domain,
    ext::{Flash, Session},
    services::Butler,
    template::Publisher,
};
use axum::{
    Form,
    extract::Query,
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use std::collections::HashMap;

pub async fn get_accounts(
    publisher: Publisher,
    butler: Butler,
    _session: Session,
    flash: Flash,
) -> Response {
    let Ok(accounts) = butler.list_all_accounts().await else {
        flash.set("Failed to load all accounts.".to_string());
        return Redirect::to("/").into_response();
    };

    publisher.account_list_page(accounts, flash.message())
}

pub async fn get_create_account(
    butler: Butler,
    publisher: Publisher,
    flash: Flash,
    _session: Session,
) -> Response {
    let Ok(currencies) = butler.list_all_currencies().await else {
        flash.set("Failed to load all currencies.".to_string());
        return Redirect::to("/").into_response();
    };
    let account_types = domain::AccountType::list_all();
    let Ok(categories) = butler.list_all_categories().await else {
        flash.set("Failed to load all categories.".to_string());
        return Redirect::to("/").into_response();
    };
    publisher.account_create_page(currencies, account_types, categories)
}

#[derive(Deserialize)]
pub struct AccountCreateForm {
    name: String,
    currency: i64,
    account_type: String,
}

pub async fn post_create_account(
    butler: Butler,
    flash: Flash,
    _session: Session,
    form: Form<AccountCreateForm>,
) -> Response {
    if let Err(e) = butler
        .new_account(form.name.clone(), form.currency, &form.account_type)
        .await
    {
        #[cfg(debug_assertions)]
        eprintln!("{:?}", e);
        flash.set("Failed to create new account.".to_string());
        return Redirect::to("/").into_response();
    }
    flash.set("New account created.".to_string());
    Redirect::to("/").into_response()
}

pub async fn get_delete_account(
    butler: Butler,
    publisher: Publisher,
    _session: Session,
    flash: Flash,
) -> Response {
    let Ok(accounts) = butler.list_all_accounts().await else {
        flash.set("Failed to load all accounts.".to_string());
        return Redirect::to("/").into_response();
    };
    publisher.account_delete_page(accounts)
}

#[derive(Deserialize)]
pub struct AccountDeleteForm {
    #[serde(default)]
    ids: Vec<i64>,
}

pub async fn post_delete_account(
    butler: Butler,
    flash: Flash,
    _session: Session,
    form: axum_extra::extract::Form<AccountDeleteForm>,
) -> Response {
    if form.ids.len() == 0 {
        return Redirect::to("/accounts/delete").into_response();
    }

    if let Err(_) = butler.drop_multiple_accounts(&form.ids).await {
        flash.set("Failed to delete some account(s).".to_string());
    } else {
        flash.set("Account(s) were deleted.".to_string());
    };
    Redirect::to("/accounts/list").into_response()
}

pub async fn get_modify_account(
    Query(param): Query<HashMap<String, String>>,
    butler: Butler,
    publisher: Publisher,
    _session: Session,
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

    let Ok(account) = butler.get_account_by_id(account_id).await else {
        flash.set("No account could be retrieved for the given id.".to_string());
        return Redirect::to("/accounts/list").into_response();
    };
    publisher.account_modify_page(account)
}

#[derive(Deserialize)]
pub struct AccountModifyForm {
    account_id: i64,
    name: String,
}

pub async fn post_modify_account(
    butler: Butler,
    flash: Flash,
    _session: Session,
    form: Form<AccountModifyForm>,
) -> Response {
    match butler
        .rename_account(form.account_id, form.name.clone())
        .await
    {
        Ok(_) => flash.set("Account renamed successfully.".to_string()),
        Err(_) => flash.set("Could not rename account.".to_string()),
    };
    Redirect::to("/accounts/list").into_response()
}
