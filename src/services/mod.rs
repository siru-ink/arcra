mod account;
mod analytics;
mod category;
mod currency;
mod session;
mod transactions;

pub use category::ServiceCategoryUpdateDataError;
pub use transactions::ListAllTransactionsError;

pub struct Butler {
    dbase: sqlx::Pool<sqlx::Sqlite>,
}

impl Butler {
    pub fn new(dbase: sqlx::Pool<sqlx::Sqlite>) -> Self {
        Self { dbase }
    }
}
