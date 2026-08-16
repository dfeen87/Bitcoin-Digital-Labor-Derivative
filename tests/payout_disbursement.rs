use bitcoin_digital_labor_derivative::disbursement::{
    DisbursementConfig, DisbursementEngine, PayoutRequest, PayoutStatus,
};
use bitcoin_digital_labor_derivative::sqlite_participant_registry::SqliteParticipantRegistry;

#[test]
fn test_payout_psbt_and_raw_tx_generation() {
    let engine = DisbursementEngine::new(DisbursementConfig::default());
    let req = PayoutRequest {
        recipient_address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
        amount_sats: 100_000,
        funding_utxo_txid: None,
        funding_utxo_vout: None,
        funding_utxo_value_sats: Some(1_000_000),
        change_address: None,
        fee_rate_sats_per_vbyte: Some(10),
        dry_run: Some(false),
    };

    let result = engine
        .create_unsigned_payout("payout-test-1".to_string(), &req)
        .expect("Failed to create payout");

    assert_eq!(result.payout_id, "payout-test-1");
    assert_eq!(result.amount_sats, 100_000);
    assert_eq!(result.status, PayoutStatus::UnsignedCreated);
    assert!(!result.psbt_base64.is_empty());
    assert!(!result.raw_tx_hex.is_empty());
    assert!(result.trust_audit.passed);
}

#[test]
fn test_ailee_trust_layer_safeguard_policy() {
    let engine = DisbursementEngine::new(DisbursementConfig::default());

    // Exceed max payout limit
    let req_exceed = PayoutRequest {
        recipient_address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
        amount_sats: 600_000_000, // exceeds 5 BTC limit
        funding_utxo_txid: None,
        funding_utxo_vout: None,
        funding_utxo_value_sats: None,
        change_address: None,
        fee_rate_sats_per_vbyte: None,
        dry_run: None,
    };

    let err = engine.create_unsigned_payout("payout-test-2".to_string(), &req_exceed);
    assert!(err.is_err());
    assert!(err
        .unwrap_err()
        .contains("AILEE Trust Layer policy check failed"));
}

#[test]
fn test_payout_sqlite_persistence() {
    let temp_db = std::env::temp_dir().join(format!("test_payouts_{}.db", uuid::Uuid::new_v4()));
    let registry =
        SqliteParticipantRegistry::open_read_write(&temp_db).expect("Failed to open read-write DB");

    let engine = DisbursementEngine::new(DisbursementConfig::default());
    let req = PayoutRequest {
        recipient_address: "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
        amount_sats: 50_000,
        funding_utxo_txid: None,
        funding_utxo_vout: None,
        funding_utxo_value_sats: Some(500_000),
        change_address: None,
        fee_rate_sats_per_vbyte: None,
        dry_run: Some(false),
    };

    let payout = engine
        .create_unsigned_payout("payout-db-1".to_string(), &req)
        .expect("Failed payout generation");

    registry.save_payout(&payout).expect("Save payout failed");

    let loaded = registry
        .get_payout_by_id("payout-db-1")
        .expect("Get payout failed")
        .expect("Payout not found");

    assert_eq!(loaded.payout_id, "payout-db-1");
    assert_eq!(loaded.amount_sats, 50_000);
    assert_eq!(
        loaded.recipient_address,
        "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
    );

    let all_payouts = registry.get_all_payouts().expect("Get all payouts failed");
    assert_eq!(all_payouts.len(), 1);

    let _ = std::fs::remove_file(temp_db);
}
