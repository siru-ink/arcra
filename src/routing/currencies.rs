use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::Form;
use serde::Deserialize;

use crate::{
    AppState,
    crumble::{Flash, Session},
    db::Currency,
    template::{CurrencyCreatePage, CurrencyDeletePage, CurrencyListPage},
};

pub async fn get_currencies(State(state): State<Arc<AppState>>, flash: Flash) -> Response {
    let currencies = Currency::list_all(&state.pool).await;
    CurrencyListPage::show(&state.tera, currencies, flash.message())
}

pub async fn get_create_currency(State(state): State<Arc<AppState>>, session: Session) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    }
    CurrencyCreatePage::show(&state.tera)
}

#[derive(Deserialize)]
pub struct CurrencyCreateForm {
    name: String,
    descriptor: String,
    symbol: String,
}

pub async fn post_create_currency(
    State(state): State<Arc<AppState>>,
    session: Session,
    flash: Flash,
    form: Form<CurrencyCreateForm>,
) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    }

    if Currency::new(&state.pool, &form.name, &form.descriptor, &form.symbol).await {
        flash.set("Created new currency successfully.".to_string());
    } else {
        flash.set("Failed to create currency. Please try again.".to_string());
    }
    Redirect::to("/currencies/list").into_response()
}

pub async fn get_delete_currency(State(state): State<Arc<AppState>>, session: Session) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    }

    let currencies = Currency::list_all(&state.pool).await;
    CurrencyDeletePage::show(&state.tera, currencies)
}

#[derive(Deserialize)]
pub struct CurrencyDeleteForm {
    id: i64,
}

pub async fn post_delete_currency(
    State(state): State<Arc<AppState>>,
    session: Session,
    flash: Flash,
    form: Form<CurrencyDeleteForm>,
) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    }

    if Currency::delete(&state.pool, form.id).await {
        flash.set("Currency deleted successfully.".to_string());
    } else {
        flash.set("Currency could not be deleted. Is it maybe still in use?".to_string());
    }
    Redirect::to("/currencies/list").into_response()
}
