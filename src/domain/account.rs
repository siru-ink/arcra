use crate::domain;

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
    ) -> Self {
        todo!()
    }
}
