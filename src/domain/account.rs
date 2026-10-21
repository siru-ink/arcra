use serde::Serialize;

use crate::domain;

#[derive(Debug, thiserror::Error)]
pub enum CreateNewAccountError {
    #[error("Name must not be empty.")]
    InvalidName,
}

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
    #[error("Failed to hydrate a Category domain object from the given inputs.")]
    InvalidCategory(#[from] domain::CategoryHydrateError),
}

#[derive(Debug, thiserror::Error)]
pub enum CreateAccountIDError {
    #[error("The provided id must be non-negative, non-zero.")]
    InvalidID,
}

pub enum FlowDirection {
    Incoming,
    Outgoing,
}

pub struct AccountID(i64);

pub struct NewAccount {
    name: String,
    currency: domain::Currency,
    account_type: domain::AccountType,
    category: domain::Category,
}

#[derive(Serialize, Clone)]
pub struct Account {
    id: i64,
    name: String,
    currency: domain::Currency,
    balance: i64,
    account_type: domain::AccountType,
    category: domain::Category,
}

#[derive(Serialize)]
pub struct RoundedCurrencyAccount {
    id: i64,
    name: String,
    currency: domain::Currency,
    balance: f64,
    account_type: domain::AccountType,
}

impl From<AccountID> for i64 {
    fn from(value: AccountID) -> Self {
        value.0
    }
}

impl From<Account> for RoundedCurrencyAccount {
    fn from(value: Account) -> Self {
        RoundedCurrencyAccount {
            id: value.id,
            name: value.name,
            currency: value.currency,
            balance: value.balance as f64 / 100.0,
            account_type: value.account_type,
        }
    }
}

impl TryFrom<&i64> for AccountID {
    type Error = CreateAccountIDError;

    fn try_from(value: &i64) -> Result<Self, Self::Error> {
        if *value > 0 {
            Ok(AccountID(*value))
        } else {
            Err(CreateAccountIDError::InvalidID)
        }
    }
}

impl TryFrom<i64> for AccountID {
    type Error = CreateAccountIDError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(AccountID(value))
        } else {
            Err(CreateAccountIDError::InvalidID)
        }
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
    pub fn category(&self) -> &domain::Category {
        &self.category
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(
        name: String,
        currency: domain::Currency,
        account_type: domain::AccountType,
        category: domain::Category,
    ) -> Result<Self, CreateNewAccountError> {
        if name.is_empty() {
            return Err(CreateNewAccountError::InvalidName);
        }

        Ok(Self {
            name,
            currency,
            account_type,
            category,
        })
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
        category_id: i64,
        category_name: String,
        category_description: String,
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

        let category =
            domain::Category::try_hydrate(category_id, category_name, category_description)?;

        Ok(Self {
            id,
            name,
            currency,
            balance,
            account_type,
            category,
        })
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn update_balance(mut self, dir: FlowDirection, amount: i64) -> Self {
        let _ = match (self.account_type, dir) {
            (
                domain::AccountType::Asset | domain::AccountType::Expense,
                FlowDirection::Outgoing,
            )
            | (
                domain::AccountType::Equity
                | domain::AccountType::Liability
                | domain::AccountType::Revenue,
                FlowDirection::Incoming,
            ) => self.balance -= amount,
            _ => self.balance += amount,
        };
        self
        // credit_account (outgoing) money being taken out
        //   asset     decrease
        //   equity    increase
        //   liability increase
        //   revenue   increase
        //   expense   decrease

        // debit_account (incoming) money being paid in
        //   asset      increase
        //   equity     decrease
        //   liability  decrease
        //   revenue    decrease
        //   expense    increase
    }

    pub fn reverse_update_balance(self, dir: FlowDirection, amount: i64) -> Self {
        self.update_balance(dir, -amount)
    }

    pub fn set_name(self, new_name: String) -> Self {
        Self {
            id: self.id,
            name: new_name,
            currency: self.currency,
            balance: self.balance,
            account_type: self.account_type,
            category: self.category,
        }
    }
}
