#[derive(Debug, thiserror::Error)]
pub enum HydrateCurrencyError {
    #[error("The provided id must be a positive integer.")]
    InvalidId,
    #[error("The provided name must not be empty.")]
    InvalidName,
    #[error("The provided descriptor must not be empty.")]
    InvalidDescriptor,
    #[error("The provided symbol must not be empty.")]
    InvalidSymbol,
}

pub struct CurrencyID(pub i64);

pub struct NewCurrency {
    name: String,
    descriptor: String,
    symbol: String,
}

pub struct Currency {
    id: i64,
    name: String,
    descriptor: String,
    symbol: String,
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
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn hydrate(
        id: i64,
        name: String,
        descriptor: String,
        symbol: String,
    ) -> Result<Self, HydrateCurrencyError> {
        if id <= 0 {
            return Err(HydrateCurrencyError::InvalidId);
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
}
