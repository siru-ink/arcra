use crate::domain;

#[derive(Debug, thiserror::Error)]
pub enum HydrateAccountError {
    #[error("The provided id must be non-negative, non-zero.")]
    InvalidID,
    #[error("The provided name must not be empty.")]
    InvalidName,
    #[error("No currency could be hydrated from the provided currency information.")]
    InvalidCurrency(#[from] domain::HydrateCurrencyError),
    #[error("No account type could be hydrated from the provided account type information.")]
    InvalidAccountType(#[from] domain::HydrateAccountTypeError),
}

#[derive(Debug, thiserror::Error)]
pub enum CreateAccountIDError {
    #[error("The provided id must be non-negative, non-zero.")]
    InvalidID,
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

impl From<AccountID> for i64 {
    fn from(value: AccountID) -> Self {
        value.0
    }
}

impl AccountID {
    pub fn new(value: i64) -> Result<Self, CreateAccountIDError> {
        if value > 0 {
            Ok(AccountID(value))
        } else {
            Err(CreateAccountIDError::InvalidID)
        }
    }
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
    pub fn balance(&self) -> i64 {
        self.balance
    }

    pub fn currency_id(&self) -> i64 {
        self.currency.id()
    }

    pub fn account_type_id(&self) -> i64 {
        self.account_type.id()
    }

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
            return Err(HydrateAccountError::InvalidID);
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

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
