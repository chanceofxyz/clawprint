use std::collections::HashSet;

use crate::sealing::candidates::HashCandidate;
use crate::sealing::db;
use rusqlite::Connection;

/// A planned batch of hashes to seal.
#[derive(Debug, Clone)]
pub struct PlannedBatch {
    pub batch_index: usize,
    pub members: Vec<HashCandidate>,
}

/// Build batches from hash candidates, deduping against existing sealed members.
pub fn build_batches(
    conn: &Connection,
    candidates: Vec<HashCandidate>,
    max_batch_size: usize,
) -> rusqlite::Result<Vec<PlannedBatch>> {
    if candidates.is_empty() || max_batch_size == 0 {
        return Ok(Vec::new());
    }

    // Get set of existing hashes from DB for dedupe
    let existing: HashSet<String> = db::all_member_hashes(conn)?;

    // Filter out already-present hashes
    let mut filtered: Vec<HashCandidate> = candidates
        .into_iter()
        .filter(|c| !existing.contains(&c.hash_hex))
        .collect();

    // Stable deterministic ordering: source_type, source_ref, hash_hex
    filtered.sort_by(|a, b| {
        let t = a.source_type.cmp(&b.source_type);
        if t != std::cmp::Ordering::Equal {
            return t;
        }
        let r = a.source_ref.cmp(&b.source_ref);
        if r != std::cmp::Ordering::Equal {
            return r;
        }
        a.hash_hex.cmp(&b.hash_hex)
    });

    // Chunk into batches
    let mut batches = Vec::new();
    let mut batch_index = 0usize;
    for chunk in filtered.chunks(max_batch_size) {
        batches.push(PlannedBatch {
            batch_index,
            members: chunk.to_vec(),
        });
        batch_index += 1;
    }

    Ok(batches)
}
