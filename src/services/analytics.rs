use crate::{db, domain};
use chrono::{DateTime, Datelike, TimeZone, Utc};

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

        fn start_of_month(dt: DateTime<Utc>) -> DateTime<Utc> {
            Utc.with_ymd_and_hms(dt.year(), dt.month(), 1, 0, 0, 0)
                .single()
                .expect("1st of any month at 00:00 is always valid")
        }

        fn start_of_year(dt: DateTime<Utc>) -> DateTime<Utc> {
            Utc.with_ymd_and_hms(dt.year(), 1, 1, 0, 0, 0)
                .single()
                .expect("Jan 1st at 00:00 is always valid")
        }

        fn time_cutoff(filter: &str) -> Option<DateTime<Utc>> {
            let now = Utc::now();
            match filter {
                "last-month" => Some(start_of_month(now)),
                "last-year" => Some(start_of_year(now)),
                _ => None,
            }
        }
        let cutoff = time_cutoff(&min_filter);
        transactions.retain(|tx| cutoff.is_none_or(|c| tx.time() >= c));

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
