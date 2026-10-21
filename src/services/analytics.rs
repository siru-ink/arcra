use crate::domain;

#[derive(Debug, thiserror::Error)]
pub enum ServiceAnalyticsCheckAccountEquationError {
    #[error("Failed to load all transactions through butler service.")]
    LoadAllTransactionsFailed(#[from] crate::services::ListAllTransactionsError),
    #[error("The equation is not balanced. Something is with the saved transactions.")]
    EquationNotBalanced,
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
                // ignore multi-currency tx that will not be balanced
                // todo! change how multi currency tx are recorded in database
                continue;
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
}
