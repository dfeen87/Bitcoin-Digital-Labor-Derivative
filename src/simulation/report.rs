use crate::rbi_engine::RbiStatus;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Clone)]
pub struct SimulationReport {
    pub scenarios: BTreeMap<String, ScenarioReport>,
}

impl SimulationReport {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct ScenarioReport {
    pub name: String,
    pub steps: Vec<StepReport>,
    pub invariants: Vec<InvariantViolation>,
}

#[derive(Debug, Serialize, Clone)]
pub struct StepReport {
    pub step_index: u64,
    pub block_height: u64,
    pub participant_count: usize,
    pub total_stake_sats: u64,
    pub average_velocity: Option<f64>,
    pub rbi_value: Option<f64>,
    pub rbi_status: Option<RbiStatus>,
    pub is_healthy: Option<bool>,
    pub demand_shock: Option<f64>,
    pub productivity_a: Option<f64>,
    pub alert: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct InvariantViolation {
    pub step_index: u64,
    pub kind: String,
    pub message: String,
}
