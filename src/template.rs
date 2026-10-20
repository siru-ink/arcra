use crate::domain;
use axum::response::{Html, IntoResponse, Response};
use tera::Context;

pub struct Publisher {
    tera: std::sync::Arc<tera::Tera>,
}

impl Publisher {
    pub fn new(tera: std::sync::Arc<tera::Tera>) -> Self {
        Self { tera }
    }

    pub fn error_page(self, error_message: &str, original_error: tera::Error) -> Response {
        eprintln!(
            "Template rendering errored: {}. {}",
            error_message, original_error
        );
        let context = Context::new();
        match self.tera.render("error.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render error.html tera template",
            )
                .into_response(),
        }
    }

    pub fn index_page(self, flash: Option<String>) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);

        if let Some(message) = flash {
            context.insert("flash", &message);
        }

        match self.tera.render("index.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <index.html> template", e),
        }
    }

    pub fn login_page(self, flash: Option<String>) -> Response {
        let mut context = Context::new();

        if let Some(message) = flash {
            context.insert("flash", &message)
        };

        match self.tera.render("login.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <login.html> template", e),
        }
    }

    pub fn logout_page(self) -> Response {
        let context = Context::new();
        match self.tera.render("logout.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <logout.html> template", e),
        }
    }

    pub fn account_list_page(
        self,
        accounts: Vec<domain::Account>,
        flash: Option<String>,
    ) -> Response {
        let accounts = accounts
            .into_iter()
            .map(domain::RoundedCurrencyAccount::from)
            .collect::<Vec<domain::RoundedCurrencyAccount>>();

        let mut context = Context::new();

        context.insert("pagenav", &true);
        context.insert("accounts", &accounts);

        if let Some(message) = flash {
            context.insert("flash", &message);
        }

        match self.tera.render("account_list.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <account_list.html> template", e),
        }
    }

    pub fn account_create_page(
        self,
        currencies: Vec<domain::Currency>,
        account_types: Vec<domain::AccountType>,
    ) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);
        context.insert("currencies", &currencies);
        context.insert("account_types", &account_types);

        match self.tera.render("account_create.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <account_create.html> template", e),
        }
    }

    pub fn account_delete_page(self, accounts: Vec<domain::Account>) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);
        context.insert("accounts", &accounts);

        match self.tera.render("account_delete.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <account_delete.html> template", e),
        }
    }

    pub fn account_modify_page(self, account: domain::Account) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);
        context.insert("account", &account);

        match self.tera.render("account_modify.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <account_modify.html> template", e),
        }
    }

    pub fn category_list_page(
        self,
        categories: Vec<domain::Category>,
        flash: Option<String>,
    ) -> Response {
        let mut context = Context::new();
        context.insert("pagenav", &true);
        context.insert("categories", &categories);
        if let Some(message) = flash {
            context.insert("flash", &message);
        }
        match self.tera.render("category_list.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <category_list.html> template.", e),
        }
    }

    pub fn category_modify_page(
        self,
        category: domain::Category,
        flash: Option<String>,
    ) -> Response {
        let mut context = Context::new();
        context.insert("pagenav", &true);
        context.insert("category", &category);
        if let Some(message) = flash {
            context.insert("flash", &message);
        }
        match self.tera.render("category_modify.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <category_modify.html> template.", e),
        }
    }

    pub fn currency_list_page(
        self,
        currencies: Vec<domain::Currency>,
        flash: Option<String>,
    ) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);
        context.insert("currencies", &currencies);

        if let Some(message) = flash {
            context.insert("flash", &message);
        }

        match self.tera.render("currency_list.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <currency_list.html> template.", e),
        }
    }

    pub fn currency_create_page(self) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);

        match self.tera.render("currency_create.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <currency_create.html> template.", e),
        }
    }

    pub fn currency_delete_page(self, currencies: Vec<domain::Currency>) -> Response {
        let mut context = Context::new();
        context.insert("pagenav", &true);
        context.insert("currencies", &currencies);
        match self.tera.render("currency_delete.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <currency_delete.html> template.", e),
        }
    }

    pub fn transaction_list_page(
        self,
        transactions: Vec<domain::Transaction>,
        flash: Option<String>,
    ) -> Response {
        let transformed_transactions = transactions
            .into_iter()
            .map(domain::RoundedCurrencyTransaction::from)
            .collect::<Vec<domain::RoundedCurrencyTransaction>>();

        let mut context = Context::new();

        context.insert("pagenav", &true);
        context.insert("transactions", &transformed_transactions);

        if let Some(message) = flash {
            context.insert("flash", &message);
        }

        match self.tera.render("transaction_list.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page("Tera failed to render <transaction_list.html> template.", e),
        }
    }

    pub fn transaction_create_page(
        self,
        accounts: Vec<domain::Account>,
        flash: Option<String>,
    ) -> Response {
        let mut context = Context::new();

        context.insert("pagenav", &true);
        context.insert("accounts", &accounts);

        if let Some(message) = flash {
            context.insert("flash", &message);
        }

        match self.tera.render("transaction_create.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page(
                "Tera failed to render <transaction_create.html> template.",
                e,
            ),
        }
    }

    pub fn transaction_delete_page(self, transaction: domain::Transaction) -> Response {
        let transaction = domain::RoundedCurrencyTransaction::from(transaction);

        let mut context = Context::new();

        context.insert("pagenav", &true);
        context.insert("transaction", &transaction);

        match self.tera.render("transaction_delete.html", &context) {
            Ok(page) => Html(page).into_response(),
            Err(e) => self.error_page(
                "Tera failed to render <transaction_delete.html> template.",
                e,
            ),
        }
    }
}
