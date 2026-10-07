use crate::{
    AppState,
    crumble::{Flash, Session},
    db::{Account, Transaction},
    template::{TransactionCreatePage, TransactionListPage},
};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::Form;
use serde::Deserialize;
use std::sync::Arc;

pub async fn get_list(
    State(state): State<Arc<AppState>>,
    session: Session,
    flash: Flash,
) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    }

    let transactions = Transaction::list_all(&state.pool).await;
    TransactionListPage::show(&state.tera, transactions, flash.message())
}

pub async fn get_create(State(state): State<Arc<AppState>>, flash: Flash) -> Response {
    let accounts = Account::list_all(&state.pool).await;
    TransactionCreatePage::show(&state.tera, accounts, flash.message())
}

#[derive(Deserialize)]
pub struct TransactionCreateForm {
    description: String,
    credit_account: i64,
    credit_amount: f64,
    debit_account: i64,
    debit_amount: f64,
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

    if form.credit_account == form.debit_account {
        flash.set("Credit and debit accounts must be different accounts.".to_string());
        return Redirect::to("/transactions/create").into_response();
    }

    let credit_amount = (form.credit_amount * 100.0).round() as i64;
    let debit_amount = (form.debit_amount * 100.0).round() as i64;

    if credit_amount <= 0 || debit_amount <= 0 {
        flash.set("Credit and debit amounts must be non-zero, positive values.".to_string());
        return Redirect::to("/transactions/create").into_response();
    }

    Transaction::new(
        &state.pool,
        &form.description,
        form.credit_account,
        credit_amount,
        form.debit_account,
        debit_amount,
    )
    .await;

    flash.set("Transaction recorded successfully.".to_string());
    Redirect::to("/transactions/list").into_response()
}
