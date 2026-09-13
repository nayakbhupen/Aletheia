use aletheia_core::{QidGrounding, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

/// Offline SQLite cache and Wikidata entity resolver
pub struct WikidataResolver {
    conn: Mutex<Connection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDetails {
    pub qid: String,
    pub label: String,
    pub description: String,
    pub aliases: Vec<String>,
}

impl WikidataResolver {
    /// Initialize with SQLite database path (in-memory or persistent file)
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)
            .map_err(|e| aletheia_core::AletheiaError::Wikidata(e.to_string()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS wikidata_entities (
                qid TEXT PRIMARY KEY,
                label TEXT NOT NULL,
                description TEXT,
                aliases_json TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| aletheia_core::AletheiaError::Wikidata(e.to_string()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS wikidata_triples (
                subject_qid TEXT NOT NULL,
                property_pid TEXT NOT NULL,
                object_qid TEXT,
                object_value TEXT NOT NULL,
                PRIMARY KEY (subject_qid, property_pid, object_value)
            )",
            [],
        )
        .map_err(|e| aletheia_core::AletheiaError::Wikidata(e.to_string()))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// In-memory resolver for fast testing & offline evaluation
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| aletheia_core::AletheiaError::Wikidata(e.to_string()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS wikidata_entities (
                qid TEXT PRIMARY KEY,
                label TEXT NOT NULL,
                description TEXT,
                aliases_json TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| aletheia_core::AletheiaError::Wikidata(e.to_string()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS wikidata_triples (
                subject_qid TEXT NOT NULL,
                property_pid TEXT NOT NULL,
                object_qid TEXT,
                object_value TEXT NOT NULL,
                PRIMARY KEY (subject_qid, property_pid, object_value)
            )",
            [],
        )
        .map_err(|e| aletheia_core::AletheiaError::Wikidata(e.to_string()))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Insert or update an entity in the local cache
    pub fn insert_entity(&self, details: &EntityDetails) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let aliases_json = serde_json::to_string(&details.aliases)?;
        conn.execute(
            "INSERT OR REPLACE INTO wikidata_entities (qid, label, description, aliases_json)
             VALUES (?1, ?2, ?3, ?4)",
            params![details.qid, details.label, details.description, aliases_json],
        )
        .map_err(|e| aletheia_core::AletheiaError::Wikidata(e.to_string()))?;
        Ok(())
    }

    /// Lookup entity from local cache
    pub fn get_entity(&self, qid: &str) -> Result<Option<EntityDetails>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT qid, label, description, aliases_json FROM wikidata_entities WHERE qid = ?1")
            .map_err(|e| aletheia_core::AletheiaError::Wikidata(e.to_string()))?;

        let mut rows = stmt
            .query(params![qid])
            .map_err(|e| aletheia_core::AletheiaError::Wikidata(e.to_string()))?;

        if let Some(row) = rows.next().map_err(|e| aletheia_core::AletheiaError::Wikidata(e.to_string()))? {
            let qid: String = row.get(0).unwrap();
            let label: String = row.get(1).unwrap();
            let description: String = row.get(2).unwrap_or_default();
            let aliases_json: String = row.get(3).unwrap();
            let aliases: Vec<String> = serde_json::from_str(&aliases_json).unwrap_or_default();

            Ok(Some(EntityDetails {
                qid,
                label,
                description,
                aliases,
            }))
        } else {
            Ok(None)
        }
    }

    /// Verify if candidate string matches ground truth entity (label or any canonical alias)
    pub fn matches_entity(&self, grounding: &QidGrounding, candidate: &str) -> bool {
        let cand_norm = candidate.trim().to_lowercase();
        let target_label = grounding.object_value.trim().to_lowercase();

        if cand_norm == target_label || cand_norm.contains(&target_label) {
            return true;
        }

        for alias in &grounding.aliases {
            let a_norm = alias.trim().to_lowercase();
            if cand_norm == a_norm || cand_norm.contains(&a_norm) {
                return true;
            }
        }

        false
    }
}
