use crate::api::types::LaborHistoryEntry;
use crate::disbursement::{
    DisbursementConfig, DisbursementEngine, PayoutRequest, PayoutTransactionResult,
};
use crate::economic_oracle::MockEconomicDataProvider;
use crate::rbi_engine::RBIEngine;
use crate::simulation::state::SimulationParticipant;
use crate::sqlite_participant_registry::SqliteParticipantRegistry;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

/// GlobalNode provides centralized access to all core protocol components.
/// This structure holds the state and services needed for the REST API.
#[derive(Clone)]
pub struct GlobalNode {
    /// RBI Engine for calculating Recession Bypass Index
    pub rbi_engine: Arc<Mutex<RBIEngine<MockEconomicDataProvider>>>,

    /// Participant registry for looking up participant data
    pub participant_registry: Option<Arc<SqliteParticipantRegistry>>,

    /// Disbursement engine for payout transaction generation & AILEE safety checks
    pub disbursement_engine: Arc<DisbursementEngine>,

    /// In-memory payouts store (when sqlite registry is absent or read-only)
    pub in_memory_payouts: Arc<RwLock<HashMap<String, PayoutTransactionResult>>>,

    /// Current pool balance in satoshis
    pub pool_balance: Arc<RwLock<u64>>,

    /// Node startup time for uptime calculation
    pub startup_time: Arc<DateTime<Utc>>,

    /// Node configuration
    pub config: Arc<NodeConfiguration>,

    /// Current block height
    pub current_block_height: Arc<RwLock<u64>>,

    /// Labor history tracking
    pub labor_history: Arc<RwLock<Vec<LaborHistoryEntry>>>,

    /// In-memory participant state
    pub participants: Arc<RwLock<Vec<SimulationParticipant>>>,
}

/// Node configuration settings
#[derive(Clone)]
pub struct NodeConfiguration {
    pub node_id: String,
    pub environment: String,
    pub rate_limit_per_minute: u32,
    pub jwt_auth_enabled: bool,
    pub cors_enabled: bool,
}

impl Default for NodeConfiguration {
    fn default() -> Self {
        Self {
            node_id: uuid::Uuid::new_v4().to_string(),
            environment: std::env::var("BDLD_ENV").unwrap_or_else(|_| "development".to_string()),
            rate_limit_per_minute: 100,
            jwt_auth_enabled: false,
            cors_enabled: true,
        }
    }
}

impl GlobalNode {
    /// Create a new GlobalNode with default components
    pub fn new() -> Self {
        let provider = MockEconomicDataProvider {
            demand_shock: 0.05,
            productivity: 1.2,
        };
        let rbi_engine = RBIEngine::new(provider);
        let disbursement_engine = DisbursementEngine::new(DisbursementConfig::default());

        Self {
            rbi_engine: Arc::new(Mutex::new(rbi_engine)),
            participant_registry: None,
            disbursement_engine: Arc::new(disbursement_engine),
            in_memory_payouts: Arc::new(RwLock::new(HashMap::new())),
            pool_balance: Arc::new(RwLock::new(0)),
            startup_time: Arc::new(Utc::now()),
            config: Arc::new(NodeConfiguration::default()),
            current_block_height: Arc::new(RwLock::new(800_000)),
            labor_history: Arc::new(RwLock::new(Vec::new())),
            participants: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Create a GlobalNode with a specific participant registry
    pub fn with_registry(mut self, registry: SqliteParticipantRegistry) -> Self {
        self.participant_registry = Some(Arc::new(registry));
        self
    }

    /// Create a GlobalNode with custom configuration
    pub fn with_config(mut self, config: NodeConfiguration) -> Self {
        self.config = Arc::new(config);
        self
    }

    /// Execute / generate a payout request
    pub fn execute_payout(&self, req: &PayoutRequest) -> Result<PayoutTransactionResult, String> {
        let payout_id = format!("payout-{}", uuid::Uuid::new_v4());
        let res = self
            .disbursement_engine
            .create_unsigned_payout(payout_id, req)?;

        // Persist to registry if available
        if let Some(ref registry) = self.participant_registry {
            let _ = registry.save_payout(&res);
        }

        // Always update in-memory cache
        if let Ok(mut payouts) = self.in_memory_payouts.write() {
            payouts.insert(res.payout_id.clone(), res.clone());
        }

        Ok(res)
    }

    /// Retrieve payout by ID
    pub fn get_payout(&self, payout_id: &str) -> Option<PayoutTransactionResult> {
        if let Some(ref registry) = self.participant_registry {
            if let Ok(Some(payout)) = registry.get_payout_by_id(payout_id) {
                return Some(payout);
            }
        }

        if let Ok(payouts) = self.in_memory_payouts.read() {
            return payouts.get(payout_id).cloned();
        }

        None
    }

    /// List all payouts
    pub fn list_payouts(&self) -> Vec<PayoutTransactionResult> {
        if let Some(ref registry) = self.participant_registry {
            if let Ok(payouts) = registry.get_all_payouts() {
                if !payouts.is_empty() {
                    return payouts;
                }
            }
        }

        if let Ok(payouts) = self.in_memory_payouts.read() {
            let mut list: Vec<PayoutTransactionResult> = payouts.values().cloned().collect();
            list.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            return list;
        }

        Vec::new()
    }

    /// Set the pool balance
    pub fn set_pool_balance(&self, balance: u64) {
        if let Ok(mut pool) = self.pool_balance.write() {
            *pool = balance;
        }
    }

    /// Get the current pool balance
    pub fn get_pool_balance(&self) -> u64 {
        self.pool_balance.read().map(|b| *b).unwrap_or(0)
    }

    /// Get uptime in seconds
    pub fn get_uptime_seconds(&self) -> u64 {
        let now = Utc::now();
        let duration = now.signed_duration_since(*self.startup_time);
        duration.num_seconds().max(0) as u64
    }

    /// Get current block height
    pub fn get_block_height(&self) -> u64 {
        self.current_block_height
            .read()
            .map(|h| *h)
            .unwrap_or(800_000)
    }

    /// Set current block height
    pub fn set_block_height(&self, height: u64) {
        if let Ok(mut h) = self.current_block_height.write() {
            *h = height;
        }
    }

    /// Add a labor history entry
    pub fn add_labor_history(&self, entry: LaborHistoryEntry) {
        if let Ok(mut history) = self.labor_history.write() {
            history.push(entry);
            // Keep only last 1000 entries
            if history.len() > 1000 {
                history.remove(0);
            }
        }
    }

    /// Get labor history (paginated)
    pub fn get_labor_history(&self, page: u32, page_size: u32) -> Vec<LaborHistoryEntry> {
        if let Ok(history) = self.labor_history.read() {
            let start = (page * page_size) as usize;
            history
                .iter()
                .skip(start)
                .take(page_size as usize)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get total history count
    pub fn get_labor_history_count(&self) -> usize {
        self.labor_history.read().map(|h| h.len()).unwrap_or(0)
    }

    /// Add a participant
    pub fn add_participant(&self, participant: SimulationParticipant) {
        if let Ok(mut participants) = self.participants.write() {
            // Remove existing participant with same ID
            participants.retain(|p| p.participant_id != participant.participant_id);
            participants.push(participant);
        }
    }

    /// Get all participants
    pub fn get_participants(&self) -> Vec<SimulationParticipant> {
        self.participants
            .read()
            .map(|p| p.clone())
            .unwrap_or_default()
    }
}

impl Default for GlobalNode {
    fn default() -> Self {
        Self::new()
    }
}
