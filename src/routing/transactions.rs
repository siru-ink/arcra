use crate::{
    ext::{Flash, Session},
    services::Butler,
    template::Publisher,
};
use axum::{
    extract::Query,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::Form;
use serde::Deserialize;
use std::collections::HashMap;

pub async fn get_list(
    butler: Butler,
    publisher: Publisher,
    _session: Session,
    flash: Flash,
) -> Response {
    let Ok(transactions) = butler.list_all_transactions().await else {
        flash.set("Could not retrieve list of transactions.".to_string());
        return Redirect::to("/").into_response();
    };
    publisher.transaction_list_page(transactions, flash.message())
}

pub async fn get_create(
    butler: Butler,
    publisher: Publisher,
    _session: Session,
    flash: Flash,
) -> Response {
    let Ok(accounts) = butler.list_all_accounts().await else {
        flash.set("Could not retrieve list of accounts.".to_string());
        return Redirect::to("/").into_response();
    };
    let Ok(categories) = butler.list_all_categories().await else {
        flash.set("Could not retrieve list of categories.".to_string());
        return Redirect::to("/").into_response();
    };
    publisher.transaction_create_page(accounts, categories, flash.message())
}

#[derive(Deserialize)]
pub struct TransactionCreateForm {
    description: String,
    credit_account: i64,
    credit_amount: f64,
    debit_account: i64,
    debit_amount: Option<f64>,
    category_id: Option<i64>,
}

pub async fn post_create(
    butler: Butler,
    flash: Flash,
    _session: Session,
    Form(form): Form<TransactionCreateForm>,
) -> Response {
    let TransactionCreateForm {
        description,
        credit_account,
        credit_amount,
        debit_account,
        debit_amount,
        category_id,
    } = form;

    match butler
        .new_transaction(
            description,
            credit_account,
            debit_account,
            credit_amount,
            debit_amount,
            category_id,
        )
        .await
    {
        Ok(_) => flash.set("Transaction saved successfully.".to_string()),
        Err(e) => {
            #[cfg(debug_assertions)]
            eprintln!("Error in transactions::post_create handler: {}", e);
            flash.set("Transaction could not be saved.".to_string());
        }
    }

    Redirect::to("/transactions/list").into_response()
}

pub async fn get_delete(
    butler: Butler,
    publisher: Publisher,
    _session: Session,
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let raw_transaction_id = match params.get("transaction_id") {
        Some(id) => id,
        None => return Redirect::to("/transactions/list").into_response(),
    };

    let transaction_id = match raw_transaction_id.parse::<i64>() {
        Ok(id) => id,
        Err(_) => return Redirect::to("/transactions/list").into_response(),
    };

    let Ok(transaction) = butler.get_transaction_by_id(transaction_id).await else {
        return Redirect::to("/transactions/list").into_response();
    };
    publisher.transaction_delete_page(transaction)
}

#[derive(Deserialize)]
pub struct TransactionDeleteForm {
    id: i64,
}

pub async fn post_delete(
    butler: Butler,
    flash: Flash,
    _session: Session,
    form: Form<TransactionDeleteForm>,
) -> Response {
    if let Err(_) = butler.drop_transaction(form.id).await {
        flash.set("Transaction could not be deleted. Please try again.".to_string());
    } else {
        flash.set("Transaction deleted successfully.".to_string());
    }

    return Redirect::to("/transactions/list").into_response();
}
