use crate::disbursement::{AileeTrustAudit, PayoutStatus, PayoutTransactionResult};
use crate::velocity_analyzer::{ParticipantRegistry, VelocityError};
use rusqlite::{params, Connection, OpenFlags};
use std::collections::HashSet;
use std::path::Path;
use std::sync::Mutex;

const REQUIRED_TABLES: &[&str] = &["participants", "participant_addresses"];
const PARTICIPANTS_COLUMNS: &[&str] = &["participant_id"];
const ADDRESS_COLUMNS: &[&str] = &["participant_id", "address", "position"];

/// Read-write or read-only registry backed by a stable SQLite schema.
#[derive(Debug)]
pub struct SqliteParticipantRegistry {
    conn: Mutex<Connection>,
    is_read_only: bool,
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
            is_read_only: true,
        })
    }

    pub fn open_read_write<P: AsRef<Path>>(path: P) -> Result<Self, VelocityError> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_URI,
        )
        .map_err(|err| VelocityError::DataSource(err.to_string()))?;

        init_schema(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
            is_read_only: false,
        })
    }

    pub fn save_payout(&self, payout: &PayoutTransactionResult) -> Result<(), VelocityError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| VelocityError::DataSource("registry lock poisoned".into()))?;

        if self.is_read_only {
            // If opened read-only, skip persistence gracefully
            return Ok(());
        }

        let trust_audit_json = serde_json::to_string(&payout.trust_audit)
            .map_err(|e| VelocityError::DataSource(e.to_string()))?;

        conn.execute(
            "INSERT OR REPLACE INTO payouts (
                payout_id, recipient_address, amount_sats, fee_sats, status,
                psbt_base64, raw_tx_hex, txid, timestamp, trust_audit_json, is_dry_run
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                payout.payout_id,
                payout.recipient_address,
                payout.amount_sats,
                payout.fee_sats,
                payout.status.to_string(),
                payout.psbt_base64,
                payout.raw_tx_hex,
                payout.txid,
                payout.timestamp,
                trust_audit_json,
                payout.is_dry_run as i32,
            ],
        )
        .map_err(|e| VelocityError::DataSource(e.to_string()))?;

        Ok(())
    }

    pub fn get_payout_by_id(
        &self,
        payout_id: &str,
    ) -> Result<Option<PayoutTransactionResult>, VelocityError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| VelocityError::DataSource("registry lock poisoned".into()))?;

        let mut stmt = conn
            .prepare(
                "SELECT payout_id, recipient_address, amount_sats, fee_sats, status, \
                        psbt_base64, raw_tx_hex, txid, timestamp, trust_audit_json, is_dry_run \
                 FROM payouts WHERE payout_id = ?1",
            )
            .map_err(|e| VelocityError::DataSource(e.to_string()))?;

        let mut rows = stmt
            .query(params![payout_id])
            .map_err(|e| VelocityError::DataSource(e.to_string()))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| VelocityError::DataSource(e.to_string()))?
        {
            let status_str: String = row.get(4).unwrap_or_default();
            let status = match status_str.as_str() {
                "pending" => PayoutStatus::Pending,
                "unsigned_created" => PayoutStatus::UnsignedCreated,
                "completed" => PayoutStatus::Completed,
                "rejected" => PayoutStatus::Rejected,
                _ => PayoutStatus::Pending,
            };

            let trust_audit_json: String = row.get(9).unwrap_or_default();
            let trust_audit: AileeTrustAudit =
                serde_json::from_str(&trust_audit_json).unwrap_or_default();

            let is_dry_run_i32: i32 = row.get(10).unwrap_or(0);

            Ok(Some(PayoutTransactionResult {
                payout_id: row
                    .get(0)
                    .map_err(|e| VelocityError::DataSource(e.to_string()))?,
                recipient_address: row
                    .get(1)
                    .map_err(|e| VelocityError::DataSource(e.to_string()))?,
                amount_sats: row
                    .get(2)
                    .map_err(|e| VelocityError::DataSource(e.to_string()))?,
                fee_sats: row
                    .get(3)
                    .map_err(|e| VelocityError::DataSource(e.to_string()))?,
                status,
                psbt_base64: row
                    .get(5)
                    .map_err(|e| VelocityError::DataSource(e.to_string()))?,
                raw_tx_hex: row
                    .get(6)
                    .map_err(|e| VelocityError::DataSource(e.to_string()))?,
                txid: row
                    .get(7)
                    .map_err(|e| VelocityError::DataSource(e.to_string()))?,
                timestamp: row
                    .get(8)
                    .map_err(|e| VelocityError::DataSource(e.to_string()))?,
                trust_audit,
                is_dry_run: is_dry_run_i32 != 0,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_payouts(&self) -> Result<Vec<PayoutTransactionResult>, VelocityError> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| VelocityError::DataSource("registry lock poisoned".into()))?;

        let mut stmt = conn
            .prepare(
                "SELECT payout_id, recipient_address, amount_sats, fee_sats, status, \
                        psbt_base64, raw_tx_hex, txid, timestamp, trust_audit_json, is_dry_run \
                 FROM payouts ORDER BY timestamp DESC",
            )
            .map_err(|e| VelocityError::DataSource(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let status_str: String = row.get(4)?;
                let status = match status_str.as_str() {
                    "pending" => PayoutStatus::Pending,
                    "unsigned_created" => PayoutStatus::UnsignedCreated,
                    "completed" => PayoutStatus::Completed,
                    "rejected" => PayoutStatus::Rejected,
                    _ => PayoutStatus::Pending,
                };

                let trust_audit_json: String = row.get(9).unwrap_or_default();
                let trust_audit: AileeTrustAudit =
                    serde_json::from_str(&trust_audit_json).unwrap_or_default();

                let is_dry_run_i32: i32 = row.get(10).unwrap_or(0);

                Ok(PayoutTransactionResult {
                    payout_id: row.get(0)?,
                    recipient_address: row.get(1)?,
                    amount_sats: row.get(2)?,
                    fee_sats: row.get(3)?,
                    status,
                    psbt_base64: row.get(5)?,
                    raw_tx_hex: row.get(6)?,
                    txid: row.get(7)?,
                    timestamp: row.get(8)?,
                    trust_audit,
                    is_dry_run: is_dry_run_i32 != 0,
                })
            })
            .map_err(|e| VelocityError::DataSource(e.to_string()))?;

        let mut payouts = Vec::new();
        for row in rows {
            payouts.push(row.map_err(|e| VelocityError::DataSource(e.to_string()))?);
        }

        Ok(payouts)
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

fn init_schema(conn: &Connection) -> Result<(), VelocityError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS participants (
            participant_id TEXT PRIMARY KEY
        )",
        [],
    )
    .map_err(|e| VelocityError::DataSource(e.to_string()))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS participant_addresses (
            participant_id TEXT NOT NULL,
            address TEXT NOT NULL,
            position INTEGER NOT NULL,
            PRIMARY KEY (participant_id, address),
            FOREIGN KEY (participant_id) REFERENCES participants(participant_id)
        )",
        [],
    )
    .map_err(|e| VelocityError::DataSource(e.to_string()))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS payouts (
            payout_id TEXT PRIMARY KEY,
            recipient_address TEXT NOT NULL,
            amount_sats INTEGER NOT NULL,
            fee_sats INTEGER NOT NULL,
            status TEXT NOT NULL,
            psbt_base64 TEXT NOT NULL,
            raw_tx_hex TEXT NOT NULL,
            txid TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            trust_audit_json TEXT NOT NULL,
            is_dry_run INTEGER NOT NULL
        )",
        [],
    )
    .map_err(|e| VelocityError::DataSource(e.to_string()))?;

    Ok(())
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
