use crate::domain;

#[derive(Debug, thiserror::Error)]
pub enum HydrateAccountError {
    #[error("The provided id must be non-negative, non-zero.")]
    InvalidId,
    #[error("The provided name must not be empty.")]
    InvalidName,
    #[error("No currency could be hydrated from the provided currency information.")]
    InvalidCurrency(#[from] domain::HydrateCurrencyError),
    #[error("No account type could be hydrated from the provided account type information.")]
    InvalidAccountType(#[from] domain::HydrateAccountTypeError),
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
        currency_id: i64,
        currency_name: String,
        currency_descriptor: String,
        currency_symbol: String,
        balance: i64,
        account_type_id: i64,
    ) -> Result<Self, HydrateAccountError> {
        if id <= 0 {
            return Err(HydrateAccountError::InvalidId);
        } else if name.is_empty() {
            return Err(HydrateAccountError::InvalidName);
        }

        let currency = domain::Currency::hydrate(
            currency_id,
            currency_name,
            currency_descriptor,
            currency_symbol,
        )?;

        let account_type = domain::AccountType::hydrate(account_type_id)?;

        Ok(Self {
            id,
            name,
            currency,
            balance,
            account_type,
        })
    }
}
