use bitcoin_digital_labor_derivative::sqlite_participant_registry::SqliteParticipantRegistry;
use rusqlite::Connection;
use std::env;
use std::fs;

#[test]
fn address_reuse_fails_open() {
    let path = env::temp_dir().join(format!(
        "participant_registry_sybil_{}.db",
        std::process::id()
    ));
    let _ = fs::remove_file(&path);

    let conn = Connection::open(&path).expect("open sqlite");
    conn.execute_batch(
        "CREATE TABLE participants (participant_id TEXT PRIMARY KEY);
         CREATE TABLE participant_addresses (
            participant_id TEXT NOT NULL,
            address TEXT NOT NULL,
            position INTEGER NOT NULL,
            PRIMARY KEY (participant_id, address)
         );
         INSERT INTO participants (participant_id) VALUES ('alice'), ('bob');
         INSERT INTO participant_addresses (participant_id, address, position) VALUES
            ('alice', 'addr-dup', 0),
            ('bob', 'addr-dup', 0);",
    )
    .expect("setup data");
    drop(conn);

    let result = SqliteParticipantRegistry::open(&path);
    assert!(result.is_err());

    let _ = fs::remove_file(&path);
}
