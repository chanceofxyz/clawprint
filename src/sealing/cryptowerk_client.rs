use crate::sealing::config::CryptowerkConfig;
use anyhow::{Result, anyhow};
use url::Url;

#[derive(Debug, Clone)]
pub struct CryptowerkClient {
    config: CryptowerkConfig,
}

#[derive(Debug, Clone)]
pub struct RegisterReceipt {
    pub retrieval_id: Option<String>,
    pub member_retrieval_id_from: Option<String>,
    pub member_retrieval_id_to: Option<String>,
}

impl CryptowerkClient {
    pub fn new(config: CryptowerkConfig) -> Self {
        Self { config }
    }

    /// Build the Cryptowerk /register URL with query params.
    ///
    /// Expected query params (bulk mode):
    /// - hashes: comma-separated hex
    /// - lookupInfos: comma-separated lookup info (same length as hashes) OR omitted if empty
    /// - mode: bulkSeal
    /// - callback: optional (if configured)
    pub fn build_register_url(
        &self,
        hashes_hex: &[String],
        lookup_infos: &[String],
    ) -> Result<String> {
        if hashes_hex.is_empty() {
            return Err(anyhow!("hashes_hex must not be empty"));
        }
        if !lookup_infos.is_empty() && lookup_infos.len() != hashes_hex.len() {
            return Err(anyhow!(
                "lookup_infos length must match hashes_hex length when provided"
            ));
        }

        // Base URL may be either:
        // - https://developers.cryptowerk.com/platform
        // - or a full path including /API/v8
        let mut base = self.config.base_url.trim_end_matches('/').to_string();

        // Ensure we include the documented path prefix if caller provided platform root.
        // If they already included /API/ in the env var, do not duplicate it.
        if !base.contains("/API/") {
            base.push_str("/API/v8");
        }

        let register_endpoint = format!("{}/register", base);
        let mut url = Url::parse(&register_endpoint)
            .map_err(|e| anyhow!("invalid Cryptowerk base url or endpoint: {e}"))?;

        // Cryptowerk expects comma-separated values for hashes / lookupInfos.
        let hashes = hashes_hex.join(",");
        url.query_pairs_mut()
            .append_pair("hashes", &hashes)
            .append_pair("mode", "bulkSeal");

        if !lookup_infos.is_empty() {
            let infos = lookup_infos.join(",");
            url.query_pairs_mut().append_pair("lookupInfos", &infos);
        }

        if let Some(callback) = &self.config.callback_spec {
            url.query_pairs_mut().append_pair("callback", callback);
        }

        Ok(url.to_string())
    }

    /// PR 3.1: no network yet.
    /// This remains a stub so CLI stays safe and tests stay green.
    pub fn register_bulk(
        &self,
        _hashes_hex: &[String],
        _lookup_infos: &[String],
    ) -> Result<RegisterReceipt> {
        Err(anyhow!(
            "Cryptowerk register_bulk not implemented yet (PR 3.1 only builds URL + tests)"
        ))
    }

    /// Header value for Cryptowerk auth.
    /// Docs specify: X-API-Key: "{api_key} {api_credential}"
    pub fn auth_header_value(&self) -> String {
        format!("{} {}", self.config.api_key, self.config.api_credential)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sealing::config::CryptowerkConfig;

    fn cfg(base_url: &str, callback: Option<&str>) -> CryptowerkConfig {
        CryptowerkConfig {
            api_key: "k".to_string(),
            api_credential: "c".to_string(),
            base_url: base_url.to_string(),
            callback_spec: callback.map(|s| s.to_string()),
        }
    }

    #[test]
    fn build_register_url_adds_api_v8_if_missing() {
        let client = CryptowerkClient::new(cfg("https://developers.cryptowerk.com/platform", None));
        let url = client
            .build_register_url(&vec!["aa".repeat(32)], &vec![])
            .unwrap();
        assert!(url.contains("/API/v8/register?"));
        assert!(url.contains("mode=bulkSeal"));
        assert!(url.contains("hashes="));
    }

    #[test]
    fn build_register_url_respects_existing_api_path() {
        let client = CryptowerkClient::new(cfg(
            "https://developers.cryptowerk.com/platform/API/v8",
            None,
        ));
        let url = client
            .build_register_url(&vec!["bb".repeat(32)], &vec![])
            .unwrap();
        // should not duplicate /API/v8
        let count = url.matches("/API/v8").count();
        assert_eq!(count, 1);
        assert!(url.contains("/API/v8/register?"));
    }

    #[test]
    fn build_register_url_includes_lookup_infos_when_provided() {
        let client = CryptowerkClient::new(cfg(
            "https://developers.cryptowerk.com/platform/API/v8",
            None,
        ));
        let hashes = vec!["11".repeat(32), "22".repeat(32)];
        let infos = vec!["hi_holger".to_string(), "from_kmac".to_string()];
        let url = client.build_register_url(&hashes, &infos).unwrap();
        assert!(url.contains("lookupInfos=hi_holger%2Cfrom_kmac"));
    }

    #[test]
    fn build_register_url_includes_callback_when_present() {
        let client = CryptowerkClient::new(cfg(
            "https://developers.cryptowerk.com/platform/API/v8",
            Some("http:jsonplain:https://example.com/callback"),
        ));
        let url = client
            .build_register_url(&vec!["cc".repeat(32)], &vec![])
            .unwrap();
        assert!(url.contains("callback="));
    }

    #[test]
    fn build_register_url_rejects_mismatched_lookup_infos() {
        let client = CryptowerkClient::new(cfg(
            "https://developers.cryptowerk.com/platform/API/v8",
            None,
        ));
        let err = client
            .build_register_url(
                &vec!["aa".repeat(32)],
                &vec!["x".to_string(), "y".to_string()],
            )
            .unwrap_err();
        assert!(err.to_string().contains("lookup_infos length must match"));
    }

    #[test]
    fn auth_header_value_format() {
        let client = CryptowerkClient::new(cfg(
            "https://developers.cryptowerk.com/platform/API/v8",
            None,
        ));
        assert_eq!(client.auth_header_value(), "k c");
    }
}
