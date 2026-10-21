use crate::{
    domain,
    ext::{Flash, Session},
    services::Butler,
    template::Publisher,
};
use axum::response::{IntoResponse, Redirect, Response};
use std::collections::BTreeMap;

pub async fn get_expense_by_category(
    butler: Butler,
    flash: Flash,
    publisher: Publisher,
    _session: Session,
) -> Response {
    let Ok(transactions) = butler.list_all_transactions().await else {
        flash.set("Could not load all transactions".to_string());
        return Redirect::to("/").into_response();
    };

    let mut totals: BTreeMap<String, BTreeMap<String, i64>> = BTreeMap::new();

    for transaction in transactions {
        let debit_account = transaction.debit_account();
        if debit_account.account_type() != domain::AccountType::Expense {
            continue;
        }
        *totals
            .entry(debit_account.category().name().to_string())
            .or_default()
            .entry(debit_account.currency().symbol().to_string())
            .or_default() += transaction.debit_amount();
    }

    publisher.analytics_expense_by_category(totals)
}
