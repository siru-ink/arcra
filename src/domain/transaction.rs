use crate::domain::{self};

#[derive(Debug, thiserror::Error)]
pub enum CreateNewTransactionError {
    #[error("Description must not be empty.")]
    InvalidDescription,
    #[error("Credit amount must be a positive integer.")]
    InvalidCreditAmount,
    #[error("Debit amount must be a positive integer.")]
    InvalidDebitAmount,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateTransactionIDError {
    #[error("The provided id must be a positive integer.")]
    InvalidID,
}

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
    #[error("Failed to hydrate contained category reference.")]
    InvalidCategory(#[from] domain::CategoryHydrateError),
}

#[derive(Clone, Copy)]
pub struct TransactionID(i64);

pub struct NewTransaction {
    description: String,
    time: chrono::DateTime<chrono::Utc>,
    credit_account: domain::Account,
    credit_amount: i64,
    debit_account: domain::Account,
    debit_amount: i64,
    category: domain::Category,
}

#[derive(serde::Serialize)]
pub struct Transaction {
    id: i64,
    description: String,
    time: chrono::DateTime<chrono::Utc>,
    credit_account: domain::Account,
    credit_amount: i64,
    debit_account: domain::Account,
    debit_amount: i64,
    category: domain::Category,
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

impl TryFrom<i64> for TransactionID {
    type Error = CreateTransactionIDError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(Self::Error::InvalidID)
        }
    }
}

impl NewTransaction {
    pub fn category(&self) -> &domain::Category {
        &self.category
    }

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

    pub fn new(
        description: String,
        credit_account: domain::Account,
        credit_amount: i64,
        debit_account: domain::Account,
        debit_amount: i64,
        category: domain::Category,
    ) -> Result<Self, CreateNewTransactionError> {
        if description.is_empty() {
            return Err(CreateNewTransactionError::InvalidDescription);
        } else if credit_amount <= 0 {
            return Err(CreateNewTransactionError::InvalidCreditAmount);
        } else if debit_amount <= 0 {
            return Err(CreateNewTransactionError::InvalidDebitAmount);
        }

        Ok(Self {
            description,
            time: chrono::Utc::now(),
            credit_account,
            credit_amount,
            debit_account,
            debit_amount,
            category,
        })
    }

    pub fn time(&self) -> chrono::DateTime<chrono::Utc> {
        self.time
    }
}

impl Transaction {
    pub fn category(&self) -> &domain::Category {
        &self.category
    }

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
        category_id: i64,
        category_name: String,
        category_description: String,
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

        let category =
            domain::Category::try_hydrate(category_id, category_name, category_description)?;

        Ok(Self {
            id,
            description,
            time,
            credit_account,
            credit_amount,
            debit_account,
            debit_amount,
            category,
        })
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn into_parts(
        self,
    ) -> (
        String,
        chrono::DateTime<chrono::Utc>,
        domain::Account,
        i64,
        domain::Account,
        i64,
    ) {
        let Transaction {
            description,
            time,
            credit_account,
            credit_amount,
            debit_account,
            debit_amount,
            ..
        } = self;
        (
            description,
            time,
            credit_account,
            credit_amount,
            debit_account,
            debit_amount,
        )
    }

    pub fn time(&self) -> chrono::DateTime<chrono::Utc> {
        self.time
    }
}
