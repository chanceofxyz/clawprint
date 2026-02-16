use rusqlite::{Connection, Result};

/// Ensure Cryptowerk sealing tables exist in the given SQLite database.
///
/// This is "CREATE TABLE IF NOT EXISTS" based so it can be called on
/// every ledger open without a separate migration framework.
pub fn ensure_sealing_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS seal_batches (
            batch_id TEXT PRIMARY KEY,
            created_at_ms INTEGER NOT NULL,
            submitted_at_ms INTEGER,
            status TEXT NOT NULL,
            api_base_url TEXT,
            callback_spec TEXT,
            hash_algorithm TEXT NOT NULL,
            hash_count INTEGER NOT NULL,
            hash_length_bytes INTEGER NOT NULL,
            cryptowerk_retrieval_id TEXT,
            cryptowerk_member_from TEXT,
            cryptowerk_member_to TEXT,
            anchor_retrieval_id TEXT,
            tag TEXT,
            bundle_method TEXT,
            seal_json TEXT,
            all_hashes_b64 TEXT,
            all_hashes_encoding TEXT,
            lookup_infos_json TEXT,
            last_error TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_seal_batches_status
            ON seal_batches(status);

        CREATE INDEX IF NOT EXISTS idx_seal_batches_cryptowerk_retrieval_id
            ON seal_batches(cryptowerk_retrieval_id);

        CREATE UNIQUE INDEX IF NOT EXISTS idx_seal_batches_anchor_retrieval_id_unique
            ON seal_batches(anchor_retrieval_id)
            WHERE anchor_retrieval_id IS NOT NULL;

        CREATE TABLE IF NOT EXISTS seal_batch_members (
            batch_id TEXT NOT NULL,
            seq INTEGER NOT NULL,
            hash_hex TEXT NOT NULL,
            lookup_info TEXT,
            source_type TEXT NOT NULL,
            source_ref TEXT,
            PRIMARY KEY (batch_id, seq),
            FOREIGN KEY (batch_id) REFERENCES seal_batches(batch_id)
                ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_seal_batch_members_hash_hex
            ON seal_batch_members(hash_hex);

        CREATE INDEX IF NOT EXISTS idx_seal_batch_members_lookup_info
            ON seal_batch_members(lookup_info);
        "#,
    )?;

    Ok(())
}
