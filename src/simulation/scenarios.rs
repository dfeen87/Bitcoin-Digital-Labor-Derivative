use crate::economic_oracle::RecordedEconomicSnapshot;
use crate::simulation::state::{
    SimulationActivity, SimulationParticipant, SimulationStepInput, SimulationUtxo,
};
use bitcoin::hashes::Hash;
use bitcoin::Txid;

#[derive(Debug, Clone)]
pub struct SimulationScenario {
    pub name: String,
    pub steps: Vec<SimulationStepInput>,
}

pub fn all_scenarios() -> Vec<SimulationScenario> {
    vec![
        zero_participation_zero_stake(),
        demand_shock_near_zero(),
        future_height_utxo_corruption(),
        address_reuse_sybil_attempt(),
        wash_activity_self_churn(),
        single_dominant_actor(),
        long_inactivity_stale_utxos(),
    ]
}

pub fn zero_participation_zero_stake() -> SimulationScenario {
    let steps = build_steps("zero_participation_zero_stake", 3, |index, height| {
        SimulationStepInput {
            step_index: index,
            block_height: height,
            total_distributed_sats: 0,
            epoch_duration_days: 1,
            participants: Vec::new(),
            utxos: Vec::new(),
            activities: Vec::new(),
            economic_snapshot: RecordedEconomicSnapshot {
                demand_shock: 0.02,
                productivity: 0.05,
            },
        }
    });

    SimulationScenario {
        name: "zero_participation_zero_stake".to_string(),
        steps,
    }
}

pub fn demand_shock_near_zero() -> SimulationScenario {
    let participant = SimulationParticipant {
        participant_id: "alice".to_string(),
        stake_sats: 100_000_000,
        trust_coefficient: 1.1,
        addresses: vec!["addr-alice".to_string()],
    };

    let steps = build_steps("demand_shock_near_zero", 2, |index, height| {
        SimulationStepInput {
            step_index: index,
            block_height: height,
            total_distributed_sats: 500_000_000,
            epoch_duration_days: 1,
            participants: vec![participant.clone()],
            utxos: vec![SimulationUtxo {
                address: "addr-alice".to_string(),
                txid: make_txid(1),
                vout: 0,
                amount_sats: 50_000_000,
                height: height.saturating_sub(5),
            }],
            activities: vec![SimulationActivity {
                address: "addr-alice".to_string(),
                count_outgoing: 3,
                volume_outgoing_sats: 10_000_000,
            }],
            economic_snapshot: RecordedEconomicSnapshot {
                demand_shock: 1e-12,
                productivity: 0.05,
            },
        }
    });

    SimulationScenario {
        name: "demand_shock_near_zero".to_string(),
        steps,
    }
}

pub fn future_height_utxo_corruption() -> SimulationScenario {
    let participant = SimulationParticipant {
        participant_id: "bob".to_string(),
        stake_sats: 75_000_000,
        trust_coefficient: 1.0,
        addresses: vec!["addr-bob".to_string()],
    };

    let steps = build_steps("future_height_utxo_corruption", 1, |index, height| {
        SimulationStepInput {
            step_index: index,
            block_height: height,
            total_distributed_sats: 250_000_000,
            epoch_duration_days: 1,
            participants: vec![participant.clone()],
            utxos: vec![SimulationUtxo {
                address: "addr-bob".to_string(),
                txid: make_txid(2),
                vout: 0,
                amount_sats: 25_000_000,
                height: height + 10,
            }],
            activities: vec![],
            economic_snapshot: RecordedEconomicSnapshot {
                demand_shock: 0.02,
                productivity: 0.03,
            },
        }
    });

    SimulationScenario {
        name: "future_height_utxo_corruption".to_string(),
        steps,
    }
}

pub fn address_reuse_sybil_attempt() -> SimulationScenario {
    let participants = vec![
        SimulationParticipant {
            participant_id: "mallory".to_string(),
            stake_sats: 50_000_000,
            trust_coefficient: 1.0,
            addresses: vec!["addr-dup".to_string()],
        },
        SimulationParticipant {
            participant_id: "mallory-clone".to_string(),
            stake_sats: 50_000_000,
            trust_coefficient: 1.0,
            addresses: vec!["addr-dup".to_string()],
        },
    ];

    let steps = build_steps("address_reuse_sybil_attempt", 1, |index, height| {
        SimulationStepInput {
            step_index: index,
            block_height: height,
            total_distributed_sats: 100_000_000,
            epoch_duration_days: 1,
            participants: participants.clone(),
            utxos: vec![],
            activities: vec![],
            economic_snapshot: RecordedEconomicSnapshot {
                demand_shock: 0.02,
                productivity: 0.04,
            },
        }
    });

    SimulationScenario {
        name: "address_reuse_sybil_attempt".to_string(),
        steps,
    }
}

pub fn wash_activity_self_churn() -> SimulationScenario {
    let participant = SimulationParticipant {
        participant_id: "churner".to_string(),
        stake_sats: 120_000_000,
        trust_coefficient: 0.9,
        addresses: vec!["addr-churn".to_string()],
    };

    let steps = build_steps("wash_activity_self_churn", 2, |index, height| {
        SimulationStepInput {
            step_index: index,
            block_height: height,
            total_distributed_sats: 400_000_000,
            epoch_duration_days: 1,
            participants: vec![participant.clone()],
            utxos: vec![SimulationUtxo {
                address: "addr-churn".to_string(),
                txid: make_txid(3),
                vout: 1,
                amount_sats: 80_000_000,
                height: height.saturating_sub(1),
            }],
            activities: vec![SimulationActivity {
                address: "addr-churn".to_string(),
                count_outgoing: 40,
                volume_outgoing_sats: 200_000_000,
            }],
            economic_snapshot: RecordedEconomicSnapshot {
                demand_shock: 0.015,
                productivity: 0.06,
            },
        }
    });

    SimulationScenario {
        name: "wash_activity_self_churn".to_string(),
        steps,
    }
}

pub fn single_dominant_actor() -> SimulationScenario {
    let participants = vec![
        SimulationParticipant {
            participant_id: "whale".to_string(),
            stake_sats: 900_000_000,
            trust_coefficient: 1.4,
            addresses: vec!["addr-whale-1".to_string(), "addr-whale-2".to_string()],
        },
        SimulationParticipant {
            participant_id: "minnow".to_string(),
            stake_sats: 10_000_000,
            trust_coefficient: 0.9,
            addresses: vec!["addr-minnow".to_string()],
        },
    ];

    let steps = build_steps("single_dominant_actor", 2, |index, height| {
        SimulationStepInput {
            step_index: index,
            block_height: height,
            total_distributed_sats: 900_000_000,
            epoch_duration_days: 1,
            participants: participants.clone(),
            utxos: vec![
                SimulationUtxo {
                    address: "addr-whale-1".to_string(),
                    txid: make_txid(4),
                    vout: 0,
                    amount_sats: 400_000_000,
                    height: height.saturating_sub(2),
                },
                SimulationUtxo {
                    address: "addr-whale-2".to_string(),
                    txid: make_txid(5),
                    vout: 1,
                    amount_sats: 300_000_000,
                    height: height.saturating_sub(3),
                },
                SimulationUtxo {
                    address: "addr-minnow".to_string(),
                    txid: make_txid(6),
                    vout: 0,
                    amount_sats: 5_000_000,
                    height: height.saturating_sub(10),
                },
            ],
            activities: vec![
                SimulationActivity {
                    address: "addr-whale-1".to_string(),
                    count_outgoing: 6,
                    volume_outgoing_sats: 100_000_000,
                },
                SimulationActivity {
                    address: "addr-minnow".to_string(),
                    count_outgoing: 1,
                    volume_outgoing_sats: 2_000_000,
                },
            ],
            economic_snapshot: RecordedEconomicSnapshot {
                demand_shock: 0.018,
                productivity: 0.04,
            },
        }
    });

    SimulationScenario {
        name: "single_dominant_actor".to_string(),
        steps,
    }
}

pub fn long_inactivity_stale_utxos() -> SimulationScenario {
    let participant = SimulationParticipant {
        participant_id: "sleeper".to_string(),
        stake_sats: 60_000_000,
        trust_coefficient: 1.0,
        addresses: vec!["addr-sleeper".to_string()],
    };

    let steps = build_steps("long_inactivity_stale_utxos", 2, |index, height| {
        SimulationStepInput {
            step_index: index,
            block_height: height,
            total_distributed_sats: 200_000_000,
            epoch_duration_days: 1,
            participants: vec![participant.clone()],
            utxos: vec![SimulationUtxo {
                address: "addr-sleeper".to_string(),
                txid: make_txid(7),
                vout: 0,
                amount_sats: 30_000_000,
                height: height.saturating_sub(50_000),
            }],
            activities: vec![SimulationActivity {
                address: "addr-sleeper".to_string(),
                count_outgoing: 0,
                volume_outgoing_sats: 0,
            }],
            economic_snapshot: RecordedEconomicSnapshot {
                demand_shock: 0.02,
                productivity: 0.01,
            },
        }
    });

    SimulationScenario {
        name: "long_inactivity_stale_utxos".to_string(),
        steps,
    }
}

fn build_steps<F>(prefix: &str, count: u64, mut builder: F) -> Vec<SimulationStepInput>
where
    F: FnMut(u64, u64) -> SimulationStepInput,
{
    let base_height = deterministic_height(prefix);
    (0..count)
        .map(|index| {
            let height = base_height + index;
            builder(index, height)
        })
        .collect()
}

fn deterministic_height(seed: &str) -> u64 {
    let mut acc = 0_u64;
    for byte in seed.as_bytes() {
        acc = acc.wrapping_mul(131).wrapping_add(*byte as u64);
    }
    100_000 + (acc % 10_000)
}

fn make_txid(seed: u8) -> Txid {
    Txid::from_slice(&[seed; 32]).expect("txid")
}
