use crate::db::{Account, AccountType, Currency};
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
    pub fn show(tera: &Tera, flash: Option<String>) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);

        if let Some(message) = flash {
            context.insert("flash", &message);
        }

        match tera.render("index.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(_) => ErrorPage::show(tera, "Tera failed to render <index.html> template"),
        }
    }
}

pub struct LoginPage {}

impl LoginPage {
    pub fn show(tera: &Tera, flash: Option<String>) -> Response {
        let mut context = Context::new();

        if let Some(message) = flash {
            context.insert("flash", &message)
        };

        match tera.render("login.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(_) => ErrorPage::show(tera, "Tera failed to render <login.html> template"),
        }
    }
}

pub struct LogoutPage {}

impl LogoutPage {
    pub fn show(tera: &Tera) -> Response {
        let context = Context::new();
        match tera.render("logout.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(_) => ErrorPage::show(tera, "Tera failed to render <logout.html> template"),
        }
    }
}

pub struct AccountListPage {}

impl AccountListPage {
    pub fn show(tera: &Tera, accounts: Vec<Account>, flash: Option<String>) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);
        context.insert("accounts", &accounts);

        if let Some(message) = flash {
            context.insert("flash", &message);
        }

        match tera.render("account_list.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(_) => ErrorPage::show(tera, "Tera failed to render <account_list.html> template"),
        }
    }
}

pub struct AccountCreatePage {}

impl AccountCreatePage {
    pub fn show(
        tera: &Tera,
        currencies: Vec<Currency>,
        account_types: Vec<AccountType>,
    ) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);
        context.insert("currencies", &currencies);
        context.insert("account_types", &account_types);

        match tera.render("account_create.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(_) => ErrorPage::show(tera, "Tera failed to render <account_create.html> template"),
        }
    }
}

pub struct AccountDeletePage {}

impl AccountDeletePage {
    pub fn show(tera: &Tera, accounts: Vec<Account>) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);
        context.insert("accounts", &accounts);

        match tera.render("account_delete.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(_) => ErrorPage::show(tera, "Tera failed to render <account_delete.html> template"),
        }
    }
}

pub struct AccountModifyPage {}

impl AccountModifyPage {
    pub fn show(tera: &Tera, account: Account) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);
        context.insert("account", &account);

        match tera.render("account_modify.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(_) => ErrorPage::show(tera, "Tera failed to render <account_modify.html> template"),
        }
    }
}
