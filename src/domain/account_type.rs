use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum HydrateAccountTypeError {
    #[error("The provided number was outside the acceptable range. Only 1-5 is accepted.")]
    InvalidId,
}

#[derive(Clone, Copy, Serialize)]
pub enum AccountType {
    Asset = 1,
    Expense,
    Equity,
    Liability,
    Revenue,
}

impl TryFrom<&str> for AccountType {
    type Error = HydrateAccountTypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Asset" => Ok(AccountType::Asset),
            "Expense" => Ok(AccountType::Expense),
            "Equity" => Ok(AccountType::Equity),
            "Liability" => Ok(AccountType::Liability),
            "Revenue" => Ok(AccountType::Revenue),
            _ => Err(HydrateAccountTypeError::InvalidId),
        }
    }
}

impl AccountType {
    pub fn id(&self) -> i64 {
        *self as i64
    }

    pub fn hydrate(id: i64) -> Result<Self, HydrateAccountTypeError> {
        match id {
            1 => Ok(AccountType::Asset),
            2 => Ok(AccountType::Expense),
            3 => Ok(AccountType::Equity),
            4 => Ok(AccountType::Liability),
            5 => Ok(AccountType::Revenue),
            _ => Err(HydrateAccountTypeError::InvalidId),
        }
    }

    pub fn list_all() -> Vec<AccountType> {
        vec![
            AccountType::Asset,
            AccountType::Expense,
            AccountType::Equity,
            AccountType::Liability,
            AccountType::Revenue,
        ]
    }
}
