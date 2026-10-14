use crate::domain;

#[derive(Debug, thiserror::Error)]
pub enum HydrateTransactionError {
    #[error("The provided id msut be a positive integer.")]
    InvalidID,
    #[error("The description must not be empty.")]
    EmptyDescription,
    #[error("Failed to hydrate contained credit account reference.")]
    InvalidCreditAccount(domain::HydrateAccountError),
    #[error("Failed to hydrate contained debit account reference.")]
    InvalidDebitAccount(domain::HydrateAccountError),
}

pub struct TransactionID(i64);

pub struct NewTransaction {
    description: String,
    time: chrono::DateTime<chrono::Utc>,
    credit_account: domain::Account,
    credit_amount: i64,
    debit_account: domain::Account,
    debit_amount: i64,
}

pub struct Transaction {
    id: i64,
    description: String,
    time: chrono::DateTime<chrono::Utc>,
    credit_account: domain::Account,
    credit_amount: i64,
    debit_account: domain::Account,
    debit_amount: i64,
}

impl From<TransactionID> for i64 {
    fn from(value: TransactionID) -> Self {
        value.0
    }
}

impl From<Transaction> for TransactionID {
    fn from(value: Transaction) -> Self {
        TransactionID(value.id)
    }
}

impl NewTransaction {
    pub fn credit_account(&self) -> &domain::Account {
        &self.credit_account
    }

    pub fn credit_amount(&self) -> i64 {
        self.credit_amount
    }

    pub fn debit_account(&self) -> &domain::Account {
        &self.debit_account
    }

    pub fn debit_amount(&self) -> i64 {
        self.debit_amount
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn time(&self) -> chrono::DateTime<chrono::Utc> {
        self.time
    }
}

impl Transaction {
    pub fn credit_account(&self) -> &domain::Account {
        &self.credit_account
    }

    pub fn credit_amount(&self) -> i64 {
        self.credit_amount
    }

    pub fn debit_account(&self) -> &domain::Account {
        &self.debit_account
    }

    pub fn debit_amount(&self) -> i64 {
        self.debit_amount
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn hydrate(
        id: i64,
        description: String,
        time: chrono::DateTime<chrono::Utc>,
        credit_account_id: i64,
        credit_account_name: String,
        credit_account_currency_id: i64,
        credit_account_currency_name: String,
        credit_account_currency_descriptor: String,
        credit_account_currency_symbol: String,
        credit_account_balance: i64,
        credit_account_account_type: i64,
        credit_amount: i64,
        debit_account_id: i64,
        debit_account_name: String,
        debit_account_currency_id: i64,
        debit_account_currency_name: String,
        debit_account_currency_descriptor: String,
        debit_account_currency_symbol: String,
        debit_account_balance: i64,
        debit_account_account_type: i64,
        debit_amount: i64,
    ) -> Result<Self, HydrateTransactionError> {
        if id <= 0 {
            return Err(HydrateTransactionError::InvalidID);
        } else if description.is_empty() {
            return Err(HydrateTransactionError::EmptyDescription);
        }

        let credit_account = domain::Account::hydrate(
            credit_account_id,
            credit_account_name,
            credit_account_currency_id,
            credit_account_currency_name,
            credit_account_currency_descriptor,
            credit_account_currency_symbol,
            credit_account_balance,
            credit_account_account_type,
        )
        .map_err(|e| HydrateTransactionError::InvalidCreditAccount(e))?;

        let debit_account = domain::Account::hydrate(
            debit_account_id,
            debit_account_name,
            debit_account_currency_id,
            debit_account_currency_name,
            debit_account_currency_descriptor,
            debit_account_currency_symbol,
            debit_account_balance,
            debit_account_account_type,
        )
        .map_err(|e| HydrateTransactionError::InvalidDebitAccount(e))?;

        Ok(Self {
            id,
            description,
            time,
            credit_account,
            credit_amount,
            debit_account,
            debit_amount,
        })
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn time(&self) -> chrono::DateTime<chrono::Utc> {
        self.time
    }
}
