#[derive(Debug, thiserror::Error)]
pub enum HydrateAccountTypeError {
    #[error("The provided number was outside the acceptable range. Only 1-5 is accepted.")]
    InvalidId,
}

#[derive(Clone, Copy)]
pub enum AccountType {
    Asset = 1,
    Expense,
    Equity,
    Liability,
    Revenue,
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
}
