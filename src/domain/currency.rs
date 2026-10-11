pub struct CurrencyID(pub i64);

pub struct NewCurrency {}

pub struct Currency {
    id: i64,
    name: String,
    descriptor: String,
    symbol: String,
}

impl Currency {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn hydrate(id: i64, name: String, descriptor: String, symbol: String) -> Self {
        todo!()
    }
}
