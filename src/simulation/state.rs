use crate::economic_oracle::RecordedEconomicSnapshot;
use bitcoin::Txid;

#[derive(Debug, Clone)]
pub struct SimulationParticipant {
    pub participant_id: String,
    pub stake_sats: u64,
    pub trust_coefficient: f64,
    pub addresses: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SimulationUtxo {
    pub address: String,
    pub txid: Txid,
    pub vout: u32,
    pub amount_sats: u64,
    pub height: u64,
}

#[derive(Debug, Clone)]
pub struct SimulationActivity {
    pub address: String,
    pub count_outgoing: u32,
    pub volume_outgoing_sats: u64,
}

#[derive(Debug, Clone)]
pub struct SimulationStepInput {
    pub step_index: u64,
    pub block_height: u64,
    pub total_distributed_sats: u64,
    pub epoch_duration_days: u32,
    pub participants: Vec<SimulationParticipant>,
    pub utxos: Vec<SimulationUtxo>,
    pub activities: Vec<SimulationActivity>,
    pub economic_snapshot: RecordedEconomicSnapshot,
}
