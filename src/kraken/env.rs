const KRAKEN_API_KEY: &str = "KRAKEN_API_KEY";
const KRAKEN_PRIVATE_KEY: &str = "KRAKEN_PRIVATE_KEY";

/// KrakenCredentials are the login credentials used by this library
/// to authenticate requests.
pub struct KrakenCredentials {
    api_key: String,
    private_key: String,
}

impl KrakenCredentials {
    pub fn new_from_env() -> Result<Self, std::env::VarError> {
        let api_key = std::env::var(KRAKEN_API_KEY)?;
        let private_key = std::env::var(KRAKEN_PRIVATE_KEY)?;
        Ok(Self::new(api_key, private_key))
    }

    pub fn new(api_key: String, private_key: String) -> Self {
        Self {
            api_key,
            private_key,
        }
    }

    pub fn api_key(&self) -> &String {
        &self.api_key
    }

    pub fn private_key(&self) -> &String {
        &self.private_key
    }
}
