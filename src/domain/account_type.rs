#[derive(Clone, Copy)]
pub enum AccountType {
    Asset,
    Expense,
    Equity,
    Liability,
    Revenue,
}

impl AccountType {
    pub fn hydrate(id: i64) -> Self {
        todo!()
    }
}
