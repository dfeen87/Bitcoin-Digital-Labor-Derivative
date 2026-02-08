use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum EconomicError {
    Provider(String),
    InvalidData(String),
}

impl std::fmt::Display for EconomicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EconomicError::Provider(e) => write!(f, "provider error: {e}"),
            EconomicError::InvalidData(e) => write!(f, "invalid data: {e}"),
        }
    }
}

impl std::error::Error for EconomicError {}

/// External economic data provider interface.
/// Keep it small, stable, and mockable.
pub trait EconomicDataProvider: Send + Sync {
    /// Demand-shock deflation rate (annualized). Example: 0.02 = 2%.
    fn demand_shock_rate(&self) -> Result<f64, EconomicError>;

    /// AI productivity expansion factor A (unitless). Example: 0.05 = 5%.
    fn productivity_expansion(&self) -> Result<f64, EconomicError>;
}

/// Cached wrapper around any provider, to avoid over-querying.
pub struct CachedProvider<P: EconomicDataProvider> {
    inner: P,
    ttl: Duration,
    cached_at: Option<Instant>,
    cached_demand_shock: Option<f64>,
    cached_productivity: Option<f64>,
}

impl<P: EconomicDataProvider> CachedProvider<P> {
    pub fn new(inner: P, ttl: Duration) -> Self {
        Self {
            inner,
            ttl,
            cached_at: None,
            cached_demand_shock: None,
            cached_productivity: None,
        }
    }

    fn refresh_if_needed(&mut self) -> Result<(), EconomicError> {
        let stale = match self.cached_at {
            None => true,
            Some(t) => t.elapsed() >= self.ttl,
        };
        if stale {
            let ds = self.inner.demand_shock_rate()?;
            let a = self.inner.productivity_expansion()?;
            if !ds.is_finite() || ds < 0.0 {
                return Err(EconomicError::InvalidData(
                    "demand_shock_rate must be finite and >= 0".into(),
                ));
            }
            if !a.is_finite() {
                return Err(EconomicError::InvalidData(
                    "productivity_expansion must be finite".into(),
                ));
            }
            self.cached_demand_shock = Some(ds);
            self.cached_productivity = Some(a);
            self.cached_at = Some(Instant::now());
        }
        Ok(())
    }
}

impl<P: EconomicDataProvider> EconomicDataProvider for CachedProvider<P> {
    fn demand_shock_rate(&self) -> Result<f64, EconomicError> {
        // NOTE: immutable self; callers should use &mut CachedProvider in practice.
        self.cached_demand_shock.ok_or_else(|| {
            EconomicError::Provider("cache not initialized; call via &mut and refresh".into())
        })
    }

    fn productivity_expansion(&self) -> Result<f64, EconomicError> {
        self.cached_productivity.ok_or_else(|| {
            EconomicError::Provider("cache not initialized; call via &mut and refresh".into())
        })
    }
}

/// Mock provider for tests/dev.
#[derive(Debug, Clone)]
pub struct MockEconomicDataProvider {
    pub demand_shock: f64,
    pub productivity: f64,
}

impl EconomicDataProvider for MockEconomicDataProvider {
    fn demand_shock_rate(&self) -> Result<f64, EconomicError> {
        Ok(self.demand_shock)
    }

    fn productivity_expansion(&self) -> Result<f64, EconomicError> {
        Ok(self.productivity)
    }
}

/// Deterministic, recorded snapshot for offline simulations.
#[derive(Debug, Clone, Copy)]
pub struct RecordedEconomicSnapshot {
    pub demand_shock: f64,
    pub productivity: f64,
}

/// Snapshot-backed provider to avoid time-based caching or external variability.
#[derive(Debug)]
pub struct RecordedEconomicProvider {
    snapshot: Mutex<RecordedEconomicSnapshot>,
}

impl RecordedEconomicProvider {
    pub fn new(snapshot: RecordedEconomicSnapshot) -> Self {
        Self {
            snapshot: Mutex::new(snapshot),
        }
    }

    pub fn set_snapshot(&self, snapshot: RecordedEconomicSnapshot) {
        if let Ok(mut guard) = self.snapshot.lock() {
            *guard = snapshot;
        }
    }

    fn get_snapshot(&self) -> Result<RecordedEconomicSnapshot, EconomicError> {
        self.snapshot
            .lock()
            .map(|guard| *guard)
            .map_err(|_| EconomicError::Provider("snapshot lock poisoned".into()))
    }
}

impl EconomicDataProvider for RecordedEconomicProvider {
    fn demand_shock_rate(&self) -> Result<f64, EconomicError> {
        Ok(self.get_snapshot()?.demand_shock)
    }

    fn productivity_expansion(&self) -> Result<f64, EconomicError> {
        Ok(self.get_snapshot()?.productivity)
    }
}
