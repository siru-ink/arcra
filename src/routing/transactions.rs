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

    //     if form.credit_account == form.debit_account {
    //         flash.set("Credit and debit accounts must be different accounts.".to_string());
    //         return Redirect::to("/transactions/create").into_response();
    //     }
    //
    //     let Ok(credit_account) = domain::Account::get_by_id(&state.pool, form.credit_account).await
    //     else {
    //         flash.set("Could not find a credit_account. Please try again.".to_string());
    //         return Redirect::to("/transactions/create").into_response();
    //     };
    //
    //     let debit_account = domain::Account::get_by_id(&state.pool, form.debit_account).await else {
    //         flash.set("Debit account does not exist in database. Please try again.".to_string());
    //         return Redirect::to("/transactions/create").into_response();
    //     };
    //
    //     let credit_amount = (form.credit_amount * 100.0).round() as i64;
    //
    //     let debit_amount: i64;
    //     if let Some(amount) = form.debit_amount {
    //         debit_amount = (amount * 100.0).round() as i64;
    //     } else if credit_account.currency() == debit_account.currency() {
    //         debit_amount = credit_amount;
    //     } else {
    //         flash.set("Debit amount must be submitted if currencies in credit and debit accounts do not match.".to_string());
    //         return Redirect::to("/transactions/create").into_response();
    //     }
    //
    //     if credit_amount <= 0 || debit_amount <= 0 {
    //         flash.set("Credit and debit amounts must be non-zero, positive values.".to_string());
    //         return Redirect::to("/transactions/create").into_response();
    //     }
    //
    //     let mut tx = match state.pool.begin().await {
    //         Ok(conn) => conn,
    //         Err(_) => {
    //             flash.set("Database could not be accessed. Please try again.".to_string());
    //             return Redirect::to("/transactions/create").into_response();
    //         }
    //     };
    //
    //     let insert_succeed = Transaction::new(
    //         &mut tx,
    //         &form.description,
    //         form.credit_account,
    //         credit_amount,
    //         form.debit_account,
    //         debit_amount,
    //     )
    //     .await;
    //     let credit_update_succeed = credit_account
    //         .update_balance(&mut tx, credit_amount)
    //         .await
    //         .is_ok();
    //     let debit_update_succeed = debit_account
    //         .update_balance(&mut tx, debit_amount)
    //         .await
    //         .is_ok();
    //
    //     if insert_succeed && credit_update_succeed && debit_update_succeed {
    //         if tx.commit().await.is_ok() {
    //             flash.set("Transaction recorded successfully.".to_string());
    //             return Redirect::to("/transactions/list").into_response();
    //         }
    //     } else {
    //         if tx.rollback().await.is_err() {
    //             eprintln!("A database transaction rollback failed.");
    //         }
    //     }
    //
    //     flash.set("Transaction could not be finalized. Please try again.".to_string());
    //     Redirect::to("/transactions/create").into_response()
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
