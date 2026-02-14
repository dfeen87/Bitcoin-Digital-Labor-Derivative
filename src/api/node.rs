use crate::economic_oracle::MockEconomicDataProvider;
use crate::rbi_engine::RBIEngine;
use crate::sqlite_participant_registry::SqliteParticipantRegistry;
use std::sync::{Arc, Mutex};

/// GlobalNode provides centralized access to all core protocol components.
/// This structure holds the state and services needed for the REST API.
#[derive(Clone)]
pub struct GlobalNode {
    /// RBI Engine for calculating Recession Bypass Index
    pub rbi_engine: Arc<Mutex<RBIEngine<MockEconomicDataProvider>>>,
    
    /// Participant registry for looking up participant data
    pub participant_registry: Option<Arc<SqliteParticipantRegistry>>,
    
    /// Current pool balance in satoshis
    pub pool_balance: Arc<std::sync::RwLock<u64>>,
}

impl GlobalNode {
    /// Create a new GlobalNode with default components
    pub fn new() -> Self {
        let provider = MockEconomicDataProvider {
            demand_shock: 0.05,
            productivity: 1.2,
        };
        let rbi_engine = RBIEngine::new(provider);
        
        Self {
            rbi_engine: Arc::new(Mutex::new(rbi_engine)),
            participant_registry: None,
            pool_balance: Arc::new(std::sync::RwLock::new(0)),
        }
    }
    
    /// Create a GlobalNode with a specific participant registry
    pub fn with_registry(mut self, registry: SqliteParticipantRegistry) -> Self {
        self.participant_registry = Some(Arc::new(registry));
        self
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
}

impl Default for GlobalNode {
    fn default() -> Self {
        Self::new()
    }
}
