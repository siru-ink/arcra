use axum::response::{IntoResponse, Redirect};

use crate::{
    ext::{Flash, Session},
    services::Butler,
    template::Publisher,
};

pub async fn get_list(
    _: Session,
    flash: Flash,
    butler: Butler,
    publisher: Publisher,
) -> axum::response::Response {
    let Ok(categories) = butler.list_all_categories().await else {
        flash.set("Failed to load all categories.".to_string());
        return Redirect::to("/").into_response();
    };
    publisher.category_list_page(categories, flash.message())
}

pub async fn get_create(_: Session) -> axum::response::Response {
    todo!()
}

pub async fn post_create(_: Session) -> axum::response::Response {
    todo!()
}

pub async fn get_modify(_: Session) -> axum::response::Response {
    todo!()
}

pub async fn post_modify(_: Session) -> axum::response::Response {
    todo!()
}

pub async fn get_delete(_: Session) -> axum::response::Response {
    todo!()
}

pub async fn post_delete(_: Session) -> axum::response::Response {
    todo!()
}
