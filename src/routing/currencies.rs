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
    domain,
    template::{CurrencyCreatePage, CurrencyDeletePage, CurrencyListPage},
};

pub async fn get_currencies(State(state): State<Arc<AppState>>, flash: Flash) -> Response {
    let Ok(currencies) = domain::Currency::list_all(&state.pool).await else {
        flash.set("Failed to retrieve data from database.".to_string());
        return Redirect::to("/").into_response();
    };
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

    match domain::Currency::create(
        &state.pool,
        form.name.clone(),
        form.descriptor.clone(),
        form.symbol.clone(),
    )
    .await
    {
        Ok(_) => flash.set("Created new currency successfully.".to_string()),
        Err(_) => flash.set("Failed to create currency. Please try again.".to_string()),
    };

    Redirect::to("/currencies/list").into_response()
}

pub async fn get_delete_currency(
    State(state): State<Arc<AppState>>,
    flash: Flash,
    session: Session,
) -> Response {
    if !session.valid().await {
        return Redirect::to("/login").into_response();
    }

    let Ok(currencies) = domain::Currency::list_all(&state.pool).await else {
        flash.set("Failed to retrieve data from database.".to_string());
        return Redirect::to("/currencies/list").into_response();
    };

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

    match domain::Currency::drop(&state.pool, form.id).await {
        Ok(_) => flash.set("Currency deleted successfully.".to_string()),
        Err(_) => flash.set("Currency could not be deleted. Is it maybe still in use?".to_string()),
    }

    Redirect::to("/currencies/list").into_response()
}
