use crate::alerts::AlertThresholds;
use crate::economic_oracle::RecordedEconomicProvider;
use crate::rbi_engine::{DistributionPoolState, ParticipantSnapshot, RBIEngine, RBISnapshot};
use crate::simulation::state::{
    SimulationActivity, SimulationParticipant, SimulationStepInput, SimulationUtxo,
};
use crate::velocity_analyzer::{
    ChainDataSource, ParticipantRegistry, TxActivity, VelocityAnalyzer, VelocityError,
};
use crate::velocity_config::VelocityConfig;
use bitcoin::util::amount::Amount;
use chrono::TimeZone;
use rust_decimal::prelude::ToPrimitive;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct StepExecution {
    pub step_index: u64,
    pub block_height: u64,
    pub participant_count: usize,
    pub total_stake_sats: u64,
    pub average_velocity: Option<f64>,
    pub rbi_snapshot: Option<RBISnapshot>,
    pub error: Option<String>,
}

pub fn execute_step(
    input: &SimulationStepInput,
    cfg: &VelocityConfig,
    thresholds: &AlertThresholds,
) -> StepExecution {
    let mut participants = input.participants.clone();
    participants.sort_by(|a, b| a.participant_id.cmp(&b.participant_id));
    for participant in &mut participants {
        participant.addresses.sort();
    }

    if let Err(err) = ensure_unique_addresses(&participants) {
        return StepExecution {
            step_index: input.step_index,
            block_height: input.block_height,
            participant_count: participants.len(),
            total_stake_sats: participants.iter().map(|p| p.stake_sats).sum(),
            average_velocity: None,
            rbi_snapshot: None,
            error: Some(err),
        };
    }

    let registry = SimRegistry::new(&participants);
    let chain = SimChain::new(&input.utxos, &input.activities);

    let mut analyzer = match VelocityAnalyzer::new(cfg.clone(), registry, chain) {
        Ok(analyzer) => analyzer,
        Err(err) => {
            return StepExecution {
                step_index: input.step_index,
                block_height: input.block_height,
                participant_count: participants.len(),
                total_stake_sats: participants.iter().map(|p| p.stake_sats).sum(),
                average_velocity: None,
                rbi_snapshot: None,
                error: Some(err.to_string()),
            }
        }
    };

    let mut total_weighted_velocity = 0.0_f64;
    let mut total_stake = 0_u64;

    for participant in &participants {
        let multiplier = match analyzer
            .calculate_velocity_multiplier(&participant.participant_id, input.block_height)
        {
            Ok(multiplier) => multiplier,
            Err(err) => {
                return StepExecution {
                    step_index: input.step_index,
                    block_height: input.block_height,
                    participant_count: participants.len(),
                    total_stake_sats: participants.iter().map(|p| p.stake_sats).sum(),
                    average_velocity: None,
                    rbi_snapshot: None,
                    error: Some(err.to_string()),
                }
            }
        };

        let multiplier_f64 = match multiplier.to_f64() {
            Some(value) => value,
            None => {
                return StepExecution {
                    step_index: input.step_index,
                    block_height: input.block_height,
                    participant_count: participants.len(),
                    total_stake_sats: participants.iter().map(|p| p.stake_sats).sum(),
                    average_velocity: None,
                    rbi_snapshot: None,
                    error: Some("velocity multiplier conversion failed".to_string()),
                }
            }
        };

        total_weighted_velocity += multiplier_f64 * participant.stake_sats as f64;
        total_stake += participant.stake_sats;
    }

    let average_velocity = if total_stake == 0 {
        0.0
    } else {
        total_weighted_velocity / total_stake as f64
    };

    let pool_state = DistributionPoolState {
        total_distributed_sats: input.total_distributed_sats,
        average_participant_velocity: average_velocity,
        epoch_duration_days: input.epoch_duration_days,
        participants: participants
            .iter()
            .map(|p| ParticipantSnapshot {
                participant_id: p.participant_id.clone(),
                stake_amount_sats: p.stake_sats,
                trust_coefficient: p.trust_coefficient,
            })
            .collect(),
    };

    let provider = RecordedEconomicProvider::new(input.economic_snapshot);
    let mut engine = RBIEngine::new(provider)
        .with_thresholds(thresholds.clone())
        .with_velocity_config(cfg.clone());

    let timestamp = chrono::Utc
        .timestamp_opt(input.step_index as i64, 0)
        .single()
        .unwrap_or_else(|| chrono::Utc.timestamp(0, 0));

    match engine.calculate_rbi_at(&pool_state, input.block_height, timestamp) {
        Ok(snapshot) => StepExecution {
            step_index: input.step_index,
            block_height: input.block_height,
            participant_count: participants.len(),
            total_stake_sats: total_stake,
            average_velocity: Some(average_velocity),
            rbi_snapshot: Some(snapshot),
            error: None,
        },
        Err(err) => StepExecution {
            step_index: input.step_index,
            block_height: input.block_height,
            participant_count: participants.len(),
            total_stake_sats: total_stake,
            average_velocity: Some(average_velocity),
            rbi_snapshot: None,
            error: Some(err.to_string()),
        },
    }
}

fn ensure_unique_addresses(participants: &[SimulationParticipant]) -> Result<(), String> {
    let mut seen = HashMap::new();
    for participant in participants {
        for address in &participant.addresses {
            if let Some(existing) = seen.insert(address.clone(), participant.participant_id.clone())
            {
                if existing != participant.participant_id {
                    return Err(format!("address reused across participants: {address}"));
                }
            }
        }
    }
    Ok(())
}

struct SimRegistry {
    addresses: BTreeMap<String, Vec<String>>,
}

impl SimRegistry {
    fn new(participants: &[SimulationParticipant]) -> Self {
        let mut addresses = BTreeMap::new();
        for participant in participants {
            let mut entries = participant.addresses.clone();
            entries.sort();
            entries.dedup();
            addresses.insert(participant.participant_id.clone(), entries);
        }
        Self { addresses }
    }
}

impl ParticipantRegistry for SimRegistry {
    fn addresses_for(&self, participant_id: &str) -> Result<Vec<String>, VelocityError> {
        self.addresses
            .get(participant_id)
            .cloned()
            .ok_or(VelocityError::ParticipantNotFound)
    }
}

struct SimChain {
    utxos_by_address: BTreeMap<String, Vec<SimulationUtxo>>,
    activity_by_address: BTreeMap<String, SimulationActivity>,
}

impl SimChain {
    fn new(utxos: &[SimulationUtxo], activities: &[SimulationActivity]) -> Self {
        let mut utxos_by_address: BTreeMap<String, Vec<SimulationUtxo>> = BTreeMap::new();
        for utxo in utxos {
            utxos_by_address
                .entry(utxo.address.clone())
                .or_default()
                .push(utxo.clone());
        }
        for utxo_list in utxos_by_address.values_mut() {
            utxo_list.sort_by(|a, b| a.txid.cmp(&b.txid).then_with(|| a.vout.cmp(&b.vout)));
        }

        let mut activity_by_address = BTreeMap::new();
        for activity in activities {
            activity_by_address.insert(activity.address.clone(), activity.clone());
        }

        Self {
            utxos_by_address,
            activity_by_address,
        }
    }

    fn aggregate_activity(&self, addresses: &[String]) -> TxActivity {
        let mut count = 0_u32;
        let mut volume = 0_u64;
        for address in addresses {
            if let Some(activity) = self.activity_by_address.get(address) {
                count = count.saturating_add(activity.count_outgoing);
                volume = volume.saturating_add(activity.volume_outgoing_sats);
            }
        }
        TxActivity {
            count_outgoing: count,
            volume_outgoing: Amount::from_sat(volume),
        }
    }
}

impl ChainDataSource for SimChain {
    fn utxos_for_addresses(
        &self,
        addresses: &[String],
    ) -> Result<Vec<crate::utxo_scoring::UtxoEntry>, VelocityError> {
        let mut entries = Vec::new();
        let mut sorted_addresses = addresses.to_vec();
        sorted_addresses.sort();
        for address in sorted_addresses {
            if let Some(list) = self.utxos_by_address.get(&address) {
                for utxo in list {
                    entries.push(crate::utxo_scoring::UtxoEntry {
                        txid: utxo.txid,
                        vout: utxo.vout,
                        amount: Amount::from_sat(utxo.amount_sats),
                        height: utxo.height,
                    });
                }
            }
        }
        Ok(entries)
    }

    fn outgoing_activity_for_addresses(
        &self,
        addresses: &[String],
        _start_height: u64,
        _end_height: u64,
    ) -> Result<TxActivity, VelocityError> {
        Ok(self.aggregate_activity(addresses))
    }
}
