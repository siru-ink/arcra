use crate::{
    ext::{Flash, Session},
    services::Butler,
    template::Publisher,
};
use axum::response::{IntoResponse, Redirect, Response};

pub async fn get_check_accounting_equation(
    butler: Butler,
    publisher: Publisher,
    _session: Session,
) -> Response {
    match butler.check_accounting_equation().await {
        Ok(_) => publisher.analytics_check_accounting_equation_page(true),
        Err(e) => {
            #[cfg(debug_assertions)]
            eprintln!(
                "Error in analytics::get_check_accounting_equation handler: {}",
                e
            );
            return publisher.analytics_check_accounting_equation_page(false);
        }
    }
}

pub async fn get_expense_by_category(
    butler: Butler,
    flash: Flash,
    publisher: Publisher,
    _session: Session,
) -> Response {
    let Ok(totals) = butler.expense_category_totals().await else {
        flash.set("Failed to analize expenses.".to_string());
        return Redirect::to("/").into_response();
    };
    publisher.analytics_expense_by_category(totals)
}
