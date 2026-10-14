use crate::domain;

#[derive(Debug, thiserror::Error)]
pub enum HydrateTransactionError {}

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
        todo!()
    }
}
