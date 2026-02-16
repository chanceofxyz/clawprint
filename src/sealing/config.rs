#[derive(Debug, Clone)]
pub struct CryptowerkConfig {
    pub api_key: String,
    pub api_credential: String,
    pub base_url: String,
    pub callback_spec: Option<String>,
}

impl CryptowerkConfig {
    /// Load from env; returns None when not configured.
    /// Env vars:
    /// - CLAWPRINT_CRYPTOWERK_API_KEY
    /// - CLAWPRINT_CRYPTOWERK_API_CREDENTIAL
    /// - CLAWPRINT_CRYPTOWERK_BASE_URL (optional)
    /// - CLAWPRINT_CRYPTOWERK_CALLBACK_SPEC (optional)
    pub fn from_env() -> Option<Self> {
        let api_key = std::env::var("CLAWPRINT_CRYPTOWERK_API_KEY").ok()?;
        let api_credential = std::env::var("CLAWPRINT_CRYPTOWERK_API_CREDENTIAL").ok()?;

        let base_url = std::env::var("CLAWPRINT_CRYPTOWERK_BASE_URL")
            .unwrap_or_else(|_| "https://developers.cryptowerk.com/platform".to_string());

        let callback_spec = std::env::var("CLAWPRINT_CRYPTOWERK_CALLBACK_SPEC").ok();

        Some(Self {
            api_key,
            api_credential,
            base_url,
            callback_spec,
        })
    }
}
