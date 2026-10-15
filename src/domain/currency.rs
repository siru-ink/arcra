use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum HydrateCurrencyError {
    #[error("The provided id must be a positive integer.")]
    InvalidID,
    #[error("The provided name must not be empty.")]
    InvalidName,
    #[error("The provided descriptor must not be empty.")]
    InvalidDescriptor,
    #[error("The provided symbol must not be empty.")]
    InvalidSymbol,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateCurrencyIDError {
    #[error("The provided id must be a positive integer.")]
    InvalidID,
}

pub struct CurrencyID(i64);

pub struct NewCurrency {
    name: String,
    descriptor: String,
    symbol: String,
}

#[derive(Serialize)]
pub struct Currency {
    id: i64,
    name: String,
    descriptor: String,
    symbol: String,
}

impl From<CurrencyID> for i64 {
    fn from(value: CurrencyID) -> Self {
        value.0
    }
}

impl From<Currency> for CurrencyID {
    fn from(value: Currency) -> Self {
        Self(value.id)
    }
}

impl CurrencyID {
    pub fn new(value: i64) -> Result<Self, CreateCurrencyIDError> {
        if value > 0 {
            Ok(CurrencyID(value))
        } else {
            Err(CreateCurrencyIDError::InvalidID)
        }
    }
}

impl NewCurrency {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

impl Currency {
    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }

    pub fn hydrate(
        id: i64,
        name: String,
        descriptor: String,
        symbol: String,
    ) -> Result<Self, HydrateCurrencyError> {
        if id <= 0 {
            return Err(HydrateCurrencyError::InvalidID);
        } else if name.is_empty() {
            return Err(HydrateCurrencyError::InvalidName);
        } else if descriptor.is_empty() {
            return Err(HydrateCurrencyError::InvalidDescriptor);
        } else if symbol.is_empty() {
            return Err(HydrateCurrencyError::InvalidSymbol);
        }

        Ok(Self {
            id,
            name,
            descriptor,
            symbol,
        })
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}
