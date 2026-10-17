use crate::services::Butler;
use crate::{
    ext::{Flash, Session},
    template::Publisher,
};
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::Form;
use serde::Deserialize;

pub async fn get_currencies(
    butler: Butler,
    publisher: Publisher,
    _session: Session,
    flash: Flash,
) -> Response {
    let Ok(currencies) = butler.list_all_currencies().await else {
        flash.set("Failed to retrieve data from database.".to_string());
        return Redirect::to("/").into_response();
    };
    publisher.currency_list_page(currencies, flash.message())
}

pub async fn get_create_currency(publisher: Publisher, _session: Session) -> Response {
    publisher.currency_create_page()
}

#[derive(Deserialize)]
pub struct CurrencyCreateForm {
    name: String,
    descriptor: String,
    symbol: String,
}

pub async fn post_create_currency(
    butler: Butler,
    _session: Session,
    flash: Flash,
    form: Form<CurrencyCreateForm>,
) -> Response {
    match butler
        .create_currency(
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
    butler: Butler,
    publisher: Publisher,
    flash: Flash,
    _session: Session,
) -> Response {
    let Ok(currencies) = butler.list_all_currencies().await else {
        flash.set("Failed to retrieve data from database.".to_string());
        return Redirect::to("/currencies/list").into_response();
    };
    publisher.currency_delete_page(currencies)
}

#[derive(Deserialize)]
pub struct CurrencyDeleteForm {
    id: i64,
}

pub async fn post_delete_currency(
    butler: Butler,
    _session: Session,
    flash: Flash,
    form: Form<CurrencyDeleteForm>,
) -> Response {
    match butler.drop_currency(form.id).await {
        Ok(_) => flash.set("Currency deleted successfully.".to_string()),
        Err(_) => flash.set("Currency could not be deleted. Is it maybe still in use?".to_string()),
    }

    Redirect::to("/currencies/list").into_response()
}
