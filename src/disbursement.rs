use bitcoin::psbt::PartiallySignedTransaction;
use bitcoin::transaction::Transaction;
use bitcoin::{Address, Network, ScriptBuf, Sequence, TxIn, TxOut, Txid, Witness};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Status of a payout request
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PayoutStatus {
    Pending,
    UnsignedCreated,
    Completed,
    Rejected,
}

impl std::fmt::Display for PayoutStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PayoutStatus::Pending => write!(f, "pending"),
            PayoutStatus::UnsignedCreated => write!(f, "unsigned_created"),
            PayoutStatus::Completed => write!(f, "completed"),
            PayoutStatus::Rejected => write!(f, "rejected"),
        }
    }
}

/// Request payload for creating/executing a payout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutRequest {
    pub recipient_address: String,
    pub amount_sats: u64,
    pub funding_utxo_txid: Option<String>,
    pub funding_utxo_vout: Option<u32>,
    pub funding_utxo_value_sats: Option<u64>,
    pub change_address: Option<String>,
    pub fee_rate_sats_per_vbyte: Option<u64>,
    pub dry_run: Option<bool>,
}

/// AILEE Trust Layer evaluation result and safety audit parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AileeTrustAudit {
    pub passed: bool,
    pub risk_score: f64,
    pub policy_version: String,
    pub warnings: Vec<String>,
    pub max_payout_sats_limit: u64,
}

impl Default for AileeTrustAudit {
    fn default() -> Self {
        Self {
            passed: true,
            risk_score: 0.05,
            policy_version: "v1.0.0-ailee-trust".to_string(),
            warnings: Vec::new(),
            max_payout_sats_limit: 500_000_000, // 5 BTC default safety ceiling
        }
    }
}

/// Result of creating an unsigned PSBT / raw payout transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutTransactionResult {
    pub payout_id: String,
    pub recipient_address: String,
    pub amount_sats: u64,
    pub fee_sats: u64,
    pub status: PayoutStatus,
    pub psbt_base64: String,
    pub raw_tx_hex: String,
    pub txid: String,
    pub timestamp: String,
    pub trust_audit: AileeTrustAudit,
    pub is_dry_run: bool,
}

/// Configuration for disbursement safeguards
#[derive(Debug, Clone)]
pub struct DisbursementConfig {
    pub network: Network,
    pub max_single_payout_sats: u64,
    pub require_ailee_trust_pass: bool,
    pub default_fee_rate: u64,
}

impl Default for DisbursementConfig {
    fn default() -> Self {
        Self {
            network: Network::Bitcoin,
            max_single_payout_sats: 500_000_000, // 5 BTC
            require_ailee_trust_pass: true,
            default_fee_rate: 10,
        }
    }
}

pub struct DisbursementEngine {
    pub config: DisbursementConfig,
}

impl DisbursementEngine {
    pub fn new(config: DisbursementConfig) -> Self {
        Self { config }
    }

    /// Evaluates AILEE Trust Layer policy safeguards on a requested payout
    pub fn evaluate_ailee_trust_policy(&self, req: &PayoutRequest) -> AileeTrustAudit {
        let mut audit = AileeTrustAudit {
            max_payout_sats_limit: self.config.max_single_payout_sats,
            ..Default::default()
        };

        // 1. Amount limit check
        if req.amount_sats > self.config.max_single_payout_sats {
            audit.passed = false;
            audit.risk_score = 0.95;
            audit.warnings.push(format!(
                "Requested amount {} sats exceeds AILEE Trust Layer max limit of {} sats",
                req.amount_sats, self.config.max_single_payout_sats
            ));
        }

        // 2. Recipient address validation
        if Address::from_str(&req.recipient_address)
            .map(|addr| addr.require_network(self.config.network))
            .is_err()
        {
            // Try unchecked network validation if strict check fails to inspect address
            if Address::from_str(&req.recipient_address).is_err() {
                audit.passed = false;
                audit.risk_score = 1.0;
                audit.warnings.push(format!(
                    "Invalid Bitcoin address for network {:?}",
                    self.config.network
                ));
            } else {
                audit.warnings.push(format!(
                    "Address network mismatch warning for network {:?}",
                    self.config.network
                ));
            }
        }

        // 3. Minimum payout check
        if req.amount_sats < 546 {
            audit.passed = false;
            audit.risk_score = 0.85;
            audit
                .warnings
                .push("Payout amount below dust limit (546 sats)".to_string());
        }

        audit
    }

    /// Creates an Unsigned PSBT and Raw Unsigned Transaction for a payout
    pub fn create_unsigned_payout(
        &self,
        payout_id: String,
        req: &PayoutRequest,
    ) -> Result<PayoutTransactionResult, String> {
        let trust_audit = self.evaluate_ailee_trust_policy(req);
        if self.config.require_ailee_trust_pass && !trust_audit.passed {
            return Err(format!(
                "AILEE Trust Layer policy check failed: {}",
                trust_audit.warnings.join("; ")
            ));
        }

        let recipient_addr = Address::from_str(&req.recipient_address)
            .map_err(|e| format!("Invalid recipient address: {e}"))?
            .assume_checked();

        let recipient_script = recipient_addr.script_pubkey();

        // Dummy/mock or real UTXO inputs
        let dummy_txid_str = req.funding_utxo_txid.clone().unwrap_or_else(|| {
            "0000000000000000000000000000000000000000000000000000000000000001".to_string()
        });
        let utxo_txid = Txid::from_str(&dummy_txid_str)
            .map_err(|e| format!("Invalid funding UTXO txid: {e}"))?;
        let utxo_vout = req.funding_utxo_vout.unwrap_or(0);
        let utxo_value = req
            .funding_utxo_value_sats
            .unwrap_or(req.amount_sats + 10_000);

        let fee_rate = req
            .fee_rate_sats_per_vbyte
            .unwrap_or(self.config.default_fee_rate);
        let estimated_vsize = 140u64; // Approx vsize for 1-in 2-out P2WPKH
        let fee_sats = fee_rate * estimated_vsize;

        if utxo_value < req.amount_sats + fee_sats {
            return Err(format!(
                "Insufficient funding UTXO value: available {} sats, required payout {} sats + fee {} sats",
                utxo_value, req.amount_sats, fee_sats
            ));
        }

        let change_sats = utxo_value - req.amount_sats - fee_sats;

        let mut outputs = vec![TxOut {
            value: req.amount_sats,
            script_pubkey: recipient_script,
        }];

        if change_sats >= 546 {
            let change_script = if let Some(ref change_addr_str) = req.change_address {
                Address::from_str(change_addr_str)
                    .map_err(|e| format!("Invalid change address: {e}"))?
                    .assume_checked()
                    .script_pubkey()
            } else {
                // Default to recipient's script for mock change if omitted
                ScriptBuf::new()
            };

            if !change_script.is_empty() {
                outputs.push(TxOut {
                    value: change_sats,
                    script_pubkey: change_script,
                });
            }
        }

        let tx = Transaction {
            version: 2,
            lock_time: bitcoin::absolute::LockTime::ZERO,
            input: vec![TxIn {
                previous_output: bitcoin::OutPoint {
                    txid: utxo_txid,
                    vout: utxo_vout,
                },
                script_sig: ScriptBuf::new(),
                sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
                witness: Witness::default(),
            }],
            output: outputs,
        };

        let psbt = PartiallySignedTransaction::from_unsigned_tx(tx.clone())
            .map_err(|e| format!("Failed to create PSBT: {e}"))?;

        let psbt_bytes = psbt.serialize();
        use base64::Engine;
        let psbt_base64 = base64::engine::general_purpose::STANDARD.encode(&psbt_bytes);

        let raw_tx_bytes = bitcoin::consensus::serialize(&tx);
        let raw_tx_hex = hex::encode(&raw_tx_bytes);
        let calculated_txid = tx.txid().to_string();

        let is_dry_run = req.dry_run.unwrap_or(false);
        let status = if is_dry_run {
            PayoutStatus::Pending
        } else {
            PayoutStatus::UnsignedCreated
        };

        Ok(PayoutTransactionResult {
            payout_id,
            recipient_address: req.recipient_address.clone(),
            amount_sats: req.amount_sats,
            fee_sats,
            status,
            psbt_base64,
            raw_tx_hex,
            txid: calculated_txid,
            timestamp: chrono::Utc::now().to_rfc3339(),
            trust_audit,
            is_dry_run,
        })
    }
}
