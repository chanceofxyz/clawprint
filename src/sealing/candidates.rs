use chrono::{DateTime, Utc};

/// A single hash candidate that could be sealed.
#[derive(Debug, Clone)]
pub struct HashCandidate {
    pub hash_hex: String,
    pub lookup_info: Option<String>,
    /// RUN_ROOT | LEDGER_ROOT | ARTIFACT (free-form string for now)
    pub source_type: String,
    /// Run id, checkpoint id, or artifact id
    pub source_ref: Option<String>,
    /// Milliseconds since Unix epoch when this hash was created
    pub created_at_ms: i64,
}

impl HashCandidate {
    pub fn new(
        hash_hex: String,
        lookup_info: Option<String>,
        source_type: String,
        source_ref: Option<String>,
        created_at: DateTime<Utc>,
    ) -> Self {
        let created_at_ms = created_at.timestamp_millis();
        Self {
            hash_hex,
            lookup_info,
            source_type,
            source_ref,
            created_at_ms,
        }
    }
}
