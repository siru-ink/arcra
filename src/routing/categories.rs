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

pub async fn get_create(
    _: Session,
    flash: Flash,
    publisher: Publisher,
) -> axum::response::Response {
    publisher.category_create_page(flash.message())
}

#[derive(serde::Deserialize)]
pub struct CategoryCreateForm {
    name: String,
    description: String,
}

pub async fn post_create(
    _: Session,
    butler: Butler,
    flash: Flash,
    axum_extra::extract::Form(form): axum_extra::extract::Form<CategoryCreateForm>,
) -> axum::response::Response {
    let CategoryCreateForm { name, description } = form;
    match butler.create_category(name, description).await {
        Ok(_) => {
            flash.set("New category created successfully.".to_string());
            return Redirect::to("/categories/list").into_response();
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            eprintln!("Error in category::post_create handler: {:?}", e);
            flash.set("Could not create new category. Please try again.".to_string());
            return Redirect::to("/categories/create").into_response();
        }
    }
}

pub async fn get_modify(
    _: Session,
    axum::extract::Path(id): axum::extract::Path<i64>,
    butler: Butler,
    flash: Flash,
    publisher: Publisher,
) -> axum::response::Response {
    let category = match butler.get_category_by_id(id).await {
        Ok(category) => category,
        Err(e) => {
            #[cfg(debug_assertions)]
            eprintln!("Error in category::get_modify handler: {:?}", e);
            flash.set("Could not modify category. Please try again.".to_string());
            return Redirect::to("/categories/list").into_response();
        }
    };
    publisher.category_modify_page(category, flash.message())
}

#[derive(serde::Deserialize)]
pub struct CategoryModifyForm {
    id: i64,
    name: Option<String>,
    description: Option<String>,
}

pub async fn post_modify(
    _: Session,
    butler: Butler,
    flash: Flash,
    axum_extra::extract::Form(form): axum_extra::extract::Form<CategoryModifyForm>,
) -> axum::response::Response {
    let CategoryModifyForm {
        id,
        name,
        description,
    } = form;
    match butler.update_category_data(id, name, description).await {
        Ok(_) => flash.set("Updated category successfully.".to_string()),
        Err(e) => {
            #[cfg(debug_assertions)]
            eprintln!("Error in category::post_modify handler: {:?}", e);
            match e {
                crate::services::ServiceCategoryUpdateDataError::InvalidUpdateData => {
                    flash.set("At least either name or description must be edited to modify the category data.".to_string());
                    return Redirect::to(&format!("/categories/modify/{}", id)).into_response();
                }
                _ => flash.set("Could not modify category. Please try again.".to_string()),
            }
        }
    };
    Redirect::to("/categories/list").into_response()
}

pub async fn get_delete(
    _: Session,
    butler: Butler,
    flash: Flash,
    publisher: Publisher,
) -> axum::response::Response {
    let categories = match butler.list_all_categories().await {
        Ok(categories) => categories,
        Err(e) => {
            #[cfg(debug_assertions)]
            eprintln!("Error in category::get_delete handler: {:?}", e);
            flash.set("Could not load all categories.".to_string());
            return Redirect::to("/").into_response();
        }
    };
    publisher.category_delete_page(categories, flash.message())
}

#[derive(serde::Deserialize)]
pub struct CategoryDeleteForm {
    id: i64,
}

pub async fn post_delete(
    _: Session,
    butler: Butler,
    flash: Flash,
    axum_extra::extract::Form(form): axum_extra::extract::Form<CategoryDeleteForm>,
) -> axum::response::Response {
    if let Err(e) = butler.drop_category(form.id).await {
        #[cfg(debug_assertions)]
        eprintln!("Error in category::post_delete handler: {:?}", e);
        flash.set("Could not delete category. Is it maybe still being used?".to_string());
    } else {
        flash.set("Deleted category successfully.".to_string());
    }
    Redirect::to("/categories/list").into_response()
}
