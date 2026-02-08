use crate::velocity_analyzer::{ParticipantRegistry, VelocityError};
use rusqlite::{params, Connection, OpenFlags};
use std::collections::HashSet;
use std::path::Path;
use std::sync::Mutex;

const REQUIRED_TABLES: &[&str] = &["participants", "participant_addresses"];
const PARTICIPANTS_COLUMNS: &[&str] = &["participant_id"];
const ADDRESS_COLUMNS: &[&str] = &["participant_id", "address", "position"];

/// Read-only registry backed by a stable SQLite schema.
///
/// Expected schema (stable, deterministic ordering via `position`):
///
/// ```sql
/// CREATE TABLE participants (
///   participant_id TEXT PRIMARY KEY
/// );
///
/// CREATE TABLE participant_addresses (
///   participant_id TEXT NOT NULL,
///   address TEXT NOT NULL,
///   position INTEGER NOT NULL,
///   PRIMARY KEY (participant_id, address),
///   FOREIGN KEY (participant_id) REFERENCES participants(participant_id)
/// );
/// ```
///
/// Notes:
/// - Opened in READ ONLY mode (no mutation).
/// - Deterministic lookup order: `position ASC, address ASC`.
/// - No background threads; single connection guarded by a mutex.
#[derive(Debug)]
pub struct SqliteParticipantRegistry {
    conn: Mutex<Connection>,
}

impl SqliteParticipantRegistry {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, VelocityError> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI,
        )
        .map_err(|err| VelocityError::DataSource(err.to_string()))?;
        validate_schema(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

impl ParticipantRegistry for SqliteParticipantRegistry {
    fn addresses_for(&self, participant_id: &str) -> Result<Vec<String>, VelocityError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| VelocityError::DataSource("registry lock poisoned".into()))?;

        let mut stmt = conn
            .prepare(
                "SELECT address FROM participant_addresses \
                 WHERE participant_id = ?1 \
                 ORDER BY position ASC, address ASC",
            )
            .map_err(|err| VelocityError::DataSource(err.to_string()))?;

        let rows = stmt
            .query_map(params![participant_id], |row| row.get::<_, String>(0))
            .map_err(|err| VelocityError::DataSource(err.to_string()))?;

        let mut addresses = Vec::new();
        for row in rows {
            addresses.push(row.map_err(|err| VelocityError::DataSource(err.to_string()))?);
        }

        if addresses.is_empty() {
            return Err(VelocityError::ParticipantNotFound);
        }

        Ok(addresses)
    }
}

fn validate_schema(conn: &Connection) -> Result<(), VelocityError> {
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type = 'table'")
        .map_err(|err| VelocityError::DataSource(err.to_string()))?;
    let table_rows = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|err| VelocityError::DataSource(err.to_string()))?;

    let mut tables = HashSet::new();
    for row in table_rows {
        let name = row.map_err(|err| VelocityError::DataSource(err.to_string()))?;
        tables.insert(name);
    }

    for required in REQUIRED_TABLES {
        if !tables.contains(*required) {
            return Err(VelocityError::InvalidData(format!(
                "missing required table: {required}"
            )));
        }
    }

    ensure_columns(conn, "participants", PARTICIPANTS_COLUMNS)?;
    ensure_columns(conn, "participant_addresses", ADDRESS_COLUMNS)?;
    ensure_unique_addresses(conn)?;
    Ok(())
}

fn ensure_columns(
    conn: &Connection,
    table_name: &str,
    required_columns: &[&str],
) -> Result<(), VelocityError> {
    let pragma = format!("PRAGMA table_info({table_name})");
    let mut stmt = conn
        .prepare(&pragma)
        .map_err(|err| VelocityError::DataSource(err.to_string()))?;
    let column_rows = stmt
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|err| VelocityError::DataSource(err.to_string()))?;

    let mut columns = HashSet::new();
    for row in column_rows {
        let name = row.map_err(|err| VelocityError::DataSource(err.to_string()))?;
        columns.insert(name);
    }

    for required in required_columns {
        if !columns.contains(*required) {
            return Err(VelocityError::InvalidData(format!(
                "missing column {required} on {table_name}"
            )));
        }
    }

    Ok(())
}

fn ensure_unique_addresses(conn: &Connection) -> Result<(), VelocityError> {
    let mut stmt = conn
        .prepare(
            "SELECT address, COUNT(DISTINCT participant_id) as cnt \
             FROM participant_addresses \
             GROUP BY address \
             HAVING cnt > 1 \
             ORDER BY address ASC \
             LIMIT 1",
        )
        .map_err(|err| VelocityError::DataSource(err.to_string()))?;
    let mut rows = stmt
        .query([])
        .map_err(|err| VelocityError::DataSource(err.to_string()))?;
    if let Some(row) = rows
        .next()
        .map_err(|err| VelocityError::DataSource(err.to_string()))?
    {
        let address: String = row
            .get(0)
            .map_err(|err| VelocityError::DataSource(err.to_string()))?;
        return Err(VelocityError::InvalidData(format!(
            "address reused across participants: {address}"
        )));
    }
    Ok(())
}
