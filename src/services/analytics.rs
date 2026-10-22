use chrono::Datelike;

use crate::{db, domain};

#[derive(Debug, thiserror::Error)]
pub enum ServiceAnalyticsCheckAccountEquationError {
    #[error("Failed to load all transactions through butler service.")]
    LoadAllTransactionsFailed(#[from] crate::services::ListAllTransactionsError),
    #[error("The equation is not balanced. Something is with the saved transactions.")]
    EquationNotBalanced,
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceAnalyticsExpenseCategoryTotalsError {
    #[error("Failed to load all transactions through butler service.")]
    LoadAllTransactionsFailed(#[from] crate::services::ListAllTransactionsError),
    #[error("Failed to obtain a database connection.")]
    DBConnectionIssue(#[from] sqlx::Error),
    #[error("Failed to retrieve null category record from database.")]
    DBReadNullCategoryFailed(#[from] db::ReadCategoryError),
}

impl crate::services::Butler {
    pub async fn check_accounting_equation(
        &self,
    ) -> Result<(), ServiceAnalyticsCheckAccountEquationError> {
        let transactions = self.list_all_transactions().await?;

        let mut debit_totals: std::collections::HashMap<domain::Currency, i64> =
            std::collections::HashMap::new();
        let mut credit_totals: std::collections::HashMap<domain::Currency, i64> =
            std::collections::HashMap::new();

        for tx in transactions {
            if tx.credit_account().currency() != tx.debit_account().currency() {
                // Multi-currency transactions are split into two single-currency
                // transactions automatically on save, so if there are any
                // multi-currency transactions, this should panic because the server
                // is in an unrecoverable, failed state
                panic!("There is a multi-currency transaction in the database.");
            }
            *debit_totals
                .entry(tx.debit_account().currency().clone())
                .or_default() += tx.debit_amount();
            *credit_totals
                .entry(tx.credit_account().currency().clone())
                .or_default() += tx.credit_amount();
        }

        for (currency, debit) in debit_totals {
            let credit = credit_totals.get(&currency).copied().unwrap_or(0);
            if debit != credit {
                return Err(ServiceAnalyticsCheckAccountEquationError::EquationNotBalanced);
            }
        }

        Ok(())
    }

    pub async fn expense_category_totals(
        &self,
        min_filter: String,
    ) -> Result<
        std::collections::BTreeMap<String, std::collections::BTreeMap<String, i64>>,
        ServiceAnalyticsExpenseCategoryTotalsError,
    > {
        let mut transactions = self.list_all_transactions().await?;

        if &min_filter == "last-month" {
            let current_time = chrono::Utc::now();
            let beginning_of_month =
                chrono::NaiveDate::from_ymd_opt(current_time.year(), current_time.month(), 1)
                    .expect("Current year and month with day 1 is always valid.")
                    .and_hms_opt(0, 0, 0)
                    .expect("0h0m0s is always valid")
                    .and_utc();

            transactions = transactions
                .into_iter()
                .filter(|tx| tx.time() >= beginning_of_month)
                .collect();
        } else if &min_filter == "last-year" {
            let current_time = chrono::Utc::now();
            let beginning_of_month = chrono::NaiveDate::from_ymd_opt(current_time.year(), 1, 1)
                .expect("Current year with month and day 1 is always valid.")
                .and_hms_opt(0, 0, 0)
                .expect("0h0m0s is always valid")
                .and_utc();

            transactions = transactions
                .into_iter()
                .filter(|tx| tx.time() >= beginning_of_month)
                .collect();
        }

        let mut conn = self.dbase.acquire().await?;
        let null_category = db::Db::read_null_category(&mut conn).await?;
        let mut totals: std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, i64>,
        > = std::collections::BTreeMap::new();

        for transaction in transactions {
            let debit_account = transaction.debit_account();
            if debit_account.account_type() != domain::AccountType::Expense {
                continue;
            }
            if *transaction.category() == null_category {
                continue;
            }
            *totals
                .entry(transaction.category().name().to_string())
                .or_default()
                .entry(debit_account.currency().symbol().to_string())
                .or_default() += transaction.debit_amount();
        }

        Ok(totals)
    }
}
