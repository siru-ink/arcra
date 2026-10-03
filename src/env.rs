pub struct EnvConfiguration {
    pub username: String,
    pub password_hash: String,
    pub cookie_key: String,
}

impl EnvConfiguration {
    pub fn get() -> Self {
        todo!()
    }
}
