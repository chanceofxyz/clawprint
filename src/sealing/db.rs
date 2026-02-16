use rusqlite::{Connection, Result, Transaction, params};
use std::collections::HashSet;

/// Row in seal_batches representing a single bulk register attempt.
#[derive(Debug, Clone)]
pub struct SealBatchRow {
    pub batch_id: String,
    pub created_at_ms: i64,
    pub status: String, // CREATED | SUBMITTED | SEALED | FAILED
    pub api_base_url: Option<String>,
    pub callback_spec: Option<String>,
    pub hash_algorithm: String, // e.g. "sha256"
    pub hash_count: i64,
    pub hash_length_bytes: i64, // usually 32
}

/// Row in seal_batch_members: manifest of what was sealed.
#[derive(Debug, Clone)]
pub struct SealBatchMemberRow {
    pub seq: i64,
    pub hash_hex: String,
    pub lookup_info: Option<String>,
    pub source_type: String, // RUN_ROOT | LEDGER_ROOT | ARTIFACT
    pub source_ref: Option<String>,
}

/// Insert a batch + all members in a single transaction.
pub fn insert_batch_with_members(
    conn: &mut Connection,
    batch: SealBatchRow,
    members: Vec<SealBatchMemberRow>,
) -> Result<()> {
    let tx = conn.transaction()?;
    insert_batch_with_members_tx(&tx, batch, members)?;
    tx.commit()?;
    Ok(())
}

fn insert_batch_with_members_tx(
    tx: &Transaction,
    batch: SealBatchRow,
    members: Vec<SealBatchMemberRow>,
) -> Result<()> {
    tx.execute(
        r#"
        INSERT INTO seal_batches (
            batch_id,
            created_at_ms,
            status,
            api_base_url,
            callback_spec,
            hash_algorithm,
            hash_count,
            hash_length_bytes
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        params![
            batch.batch_id,
            batch.created_at_ms,
            batch.status,
            batch.api_base_url,
            batch.callback_spec,
            batch.hash_algorithm,
            batch.hash_count,
            batch.hash_length_bytes,
        ],
    )?;

    let mut stmt = tx.prepare(
        r#"
        INSERT INTO seal_batch_members (
            batch_id,
            seq,
            hash_hex,
            lookup_info,
            source_type,
            source_ref
        ) VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )?;

    for m in members {
        stmt.execute(params![
            batch.batch_id,
            m.seq,
            m.hash_hex,
            m.lookup_info,
            m.source_type,
            m.source_ref,
        ])?;
    }

    Ok(())
}

/// Return all existing member hashes (hash_hex) for dedupe.
pub fn all_member_hashes(conn: &Connection) -> Result<HashSet<String>> {
    let mut stmt = conn.prepare("SELECT hash_hex FROM seal_batch_members")?;
    let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
    let mut set = HashSet::new();
    for row in rows {
        let h = row?;
        set.insert(h);
    }
    Ok(set)
}
