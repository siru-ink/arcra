use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use tera::{Context, Tera};

pub struct ErrorPage {}

impl ErrorPage {
    pub fn show(tera: &Tera, error_message: &str) -> Response {
        eprintln!("Template rendering errored: {}", error_message);
        let context = Context::new();
        match tera.render("error.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render error.html tera template",
            )
                .into_response(),
        }
    }
}

pub struct IndexPage {}

impl IndexPage {
    pub fn show(tera: &Tera) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);

        match tera.render("index.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(_) => ErrorPage::show(tera, "Tera failed to render <index.html> template"),
        }
    }
}
