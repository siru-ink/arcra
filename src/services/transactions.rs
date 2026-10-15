// use crate::db::ReadAccountError;
//
// #[derive(Debug, thiserror::Error)]
// pub enum RegisterTransactionError {
//     #[error("Something went wrong trying to access the database.")]
//     SqlxError(#[from] sqlx::Error),
//     #[error("No account could be found with the given credit account ID.")]
//     CreditAccountNotFound(#[from] ReadAccountError),
//     #[error("The credit and debit account IDs must reference different accounts.")]
//     AccountsNotDifferent,
//     #[error("Debit amount must be provided if referenced accounts have different currencies.")]
//     MissingDebitAmount,
//     #[error("Credit amount must be non-negative, non-zero.")]
//     NegativeCreditAmount,
//     #[error("Debit amount must be non-negative, non-zero")]
//     NegativeDebitAmount,
// }
//
// #[derive(Debug, thiserror::Error)]
// pub enum DropTransactionError {}
//
// pub async fn register_transaction(
//     pool: &sqlx::SqlitePool,
//     credit_account_id: i64,
//     credit_amount: i64,
//     debit_account_id: i64,
//     debit_amount: Option<i64>,
// ) -> Result<(), RegisterTransactionError> {
//     let mut tx = pool.begin().await?;
//     // Assume that all inputs are correct?
//     //
//     // Constraints:
//     //  - The two accounts must differ, both accounts must exist
//     //  - Either:
//     //      - Both accounts have same currency and at least one value for credit account exists
//     //      - Accounts have different currencies -> both depit and credit amounts exist
//     //  - The amounts need to be non-negative, non-zero
//
//     let credit_account = domain::Account::read(&mut tx, credit_account_id).await?;
//     let debit_account = Account::read(&mut tx, debit_account_id).await?;
//
//     if credit_account == debit_account {
//         return Err(RegisterTransactionError::AccountsNotDifferent);
//     }
//
//     let debit_amount = match debit_amount {
//         Some(amount) => amount,
//         None if credit_account.currency() == debit_account.currency() => credit_amount,
//         None => return Err(RegisterTransactionError::MissingDebitAmount),
//     };
//
//     if credit_amount <= 0 {
//         return Err(RegisterTransactionError::NegativeCreditAmount);
//     } else if debit_amount <= 0 {
//         return Err(RegisterTransactionError::NegativeDebitAmount);
//     }
//
//     todo!();
// }
//
// pub async fn drop_transaction() -> Result<(), DropTransactionError> {
//     todo!()
// }
