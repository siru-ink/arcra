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
