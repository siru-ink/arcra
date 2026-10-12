use crate::domain;

#[derive(Debug, thiserror::Error)]
pub enum HydrateAccountError {
    #[error("The provided id must be non-negative, non-zero.")]
    InvalidId,
    #[error("The provided name must not be empty.")]
    InvalidName,
}

pub struct AccountID(i64);

pub struct NewAccount {
    name: String,
    currency: domain::Currency,
    account_type: domain::AccountType,
}

pub struct Account {
    id: i64,
    name: String,
    currency: domain::Currency,
    balance: i64,
    account_type: domain::AccountType,
}

impl NewAccount {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn currency(&self) -> &domain::Currency {
        &self.currency
    }

    pub fn account_type(&self) -> domain::AccountType {
        self.account_type
    }
}

impl Account {
    pub fn hydrate(
        id: i64,
        name: String,
        currency: domain::Currency,
        balance: i64,
        account_type: domain::AccountType,
    ) -> Result<Self, HydrateAccountError> {
        if id <= 0 {
            return Err(HydrateAccountError::InvalidId);
        } else if name.is_empty() {
            return Err(HydrateAccountError::InvalidName);
        }

        Ok(Self {
            id,
            name,
            currency,
            balance,
            account_type,
        })
    }
}
