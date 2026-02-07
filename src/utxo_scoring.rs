use bitcoin::Txid;
use bitcoin::util::amount::Amount;

/// Basic UTXO entry needed for freshness computations.
/// Height is when the UTXO was created.
#[derive(Debug, Clone)]
pub struct UtxoEntry {
    pub txid: Txid,
    pub vout: u32,
    pub amount: Amount,
    pub height: u64,
}

/// Compute the value-weighted average UTXO age in days.
///
/// - Weighted by satoshi value to reflect economic significance.
/// - Uses a block-time approximation (10 minutes/block).
pub fn weighted_utxo_age_days(utxos: &[UtxoEntry], current_height: u64) -> f64 {
    if utxos.is_empty() {
        return 0.0;
    }

    const BLOCK_TO_DAYS: f64 = 10.0 / 60.0 / 24.0;

    let mut total_sats: u128 = 0;
    let mut weighted_sum_days: f64 = 0.0;

    for u in utxos {
        let sats = u.amount.to_sat() as u128;
        if sats == 0 {
            continue;
        }

        let age_blocks = current_height.saturating_sub(u.height);
        // 10 min/block -> days
        let age_days = (age_blocks as f64) * BLOCK_TO_DAYS;

        total_sats += sats;
        weighted_sum_days += (sats as f64) * age_days;
    }

    if total_sats == 0 {
        0.0
    } else {
        weighted_sum_days / (total_sats as f64)
    }
}

/// Convert a weighted UTXO age (days) into a freshness score in [0, 1].
///
/// Uses a simple decay curve:
///   freshness = 1 / (1 + age_months) ; age_months = age_days / 30
///
/// Examples:
///   0d -> 1.0
///   30d -> 0.5
///   90d -> 0.25
pub fn utxo_freshness_score(age_days: f64) -> f64 {
    let age_days = age_days.max(0.0);
    let age_months = age_days / 30.0;
    let s = 1.0 / (1.0 + age_months);
    s.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::hashes::Hash;

    #[test]
    fn test_freshness_curve() {
        assert!((utxo_freshness_score(0.0) - 1.0).abs() < 1e-9);
        assert!((utxo_freshness_score(30.0) - 0.5).abs() < 1e-9);
        assert!((utxo_freshness_score(90.0) - 0.25).abs() < 1e-9);
    }

    #[test]
    fn test_weighted_age() {
        let current_height = 1000;
        let txid = Txid::from_slice(&[1u8; 32]).unwrap();

        let utxos = vec![
            UtxoEntry { txid, vout: 0, amount: Amount::from_sat(100_000_000), height: 900 }, // older, big
            UtxoEntry { txid, vout: 1, amount: Amount::from_sat(10_000_000), height: 990 },  // newer, small
        ];

        let age = weighted_utxo_age_days(&utxos, current_height);
        assert!(age > 0.0);
    }
}
