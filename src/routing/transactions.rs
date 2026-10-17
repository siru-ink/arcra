use crate::{
    AppState,
    crumble::{Flash, Session},
    domain,
    template::{TransactionCreatePage, TransactionDeletePage, TransactionListPage},
};
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::Form;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};

pub async fn get_list(
    State(state): State<Arc<AppState>>,
    session: Session,
    flash: Flash,
) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    }

    let Ok(transactions) = domain::Transaction::list_all(&state.pool).await else {
        flash.set("Could not retrieve list of transactions.".to_string());
        return Redirect::to("/").into_response();
    };

    TransactionListPage::show(&state.tera, transactions, flash.message())
}

pub async fn get_create(State(state): State<Arc<AppState>>, flash: Flash) -> Response {
    let Ok(accounts) = domain::Account::list_all(&state.pool).await else {
        flash.set("Could not retrieve list of accounts.".to_string());
        return Redirect::to("/").into_response();
    };

    TransactionCreatePage::show(&state.tera, accounts, flash.message())
}

#[derive(Deserialize)]
pub struct TransactionCreateForm {
    description: String,
    credit_account: i64,
    credit_amount: f64,
    debit_account: i64,
    debit_amount: Option<f64>,
}

pub async fn post_create(
    State(state): State<Arc<AppState>>,
    flash: Flash,
    session: Session,
    form: Form<TransactionCreateForm>,
) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    }

    match domain::Transaction::new(
        &state.pool,
        form.description.clone(),
        form.credit_account,
        form.debit_account,
        form.credit_amount,
        form.debit_amount,
    )
    .await
    {
        Ok(_) => flash.set("Transaction saved successfully.".to_string()),
        Err(_) => flash.set("Transaction could not be saved.".to_string()),
    }

    Redirect::to("/transactions/list").into_response()
}

pub async fn get_delete(
    State(state): State<Arc<AppState>>,
    session: Session,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    }

    let raw_transaction_id = match params.get("transaction_id") {
        Some(id) => id,
        None => return Redirect::to("/transactions/list").into_response(),
    };

    let transaction_id = match raw_transaction_id.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return Redirect::to("/transactions/list").into_response(),
    };

    let Ok(transaction) = domain::Transaction::get_by_id(&state.pool, transaction_id).await else {
        return Redirect::to("/transactions/list").into_response();
    };

    TransactionDeletePage::show(&state.tera, transaction)
}

#[derive(Deserialize)]
pub struct TransactionDeleteForm {
    id: i64,
}

pub async fn post_delete(
    State(state): State<Arc<AppState>>,
    flash: Flash,
    form: Form<TransactionDeleteForm>,
) -> Response {
    if let Err(_) = domain::Transaction::drop(&state.pool, form.id).await {
        flash.set("Transaction could not be deleted. Please try again.".to_string());
    } else {
        flash.set("Transaction deleted successfully.".to_string());
    }

    return Redirect::to("/transactions/list").into_response();
}
