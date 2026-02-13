use crate::utxo_scoring::UtxoEntry;
use crate::velocity_analyzer::{ChainDataSource, TxActivity, VelocityError};
use bitcoin::amount::Amount;
use bitcoin::address::NetworkUnchecked;
use bitcoin::Address;
use bitcoincore_rpc::json::GetTransactionResultDetailCategory;
use bitcoincore_rpc::{Client, RpcApi};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tracing::{debug, info};

/// Bitcoin Core-backed chain data source.
#[derive(Debug, Clone)]
pub struct BitcoinCoreChainDataSource {
    client: Arc<Client>,
    rpc_config: RpcConfig,
    cache: Arc<Mutex<CacheState>>,
    metrics: Arc<Metrics>,
}

impl BitcoinCoreChainDataSource {
    pub fn new(client: Client) -> Self {
        Self::new_with_config(client, RpcConfig::default())
    }

    pub fn new_with_config(client: Client, rpc_config: RpcConfig) -> Self {
        Self {
            client: Arc::new(client),
            rpc_config,
            cache: Arc::new(Mutex::new(CacheState::default())),
            metrics: Arc::new(Metrics::default()),
        }
    }

    fn parse_addresses(&self, addresses: &[String]) -> Result<Vec<Address>, VelocityError> {
        addresses
            .iter()
            .map(|addr| {
                Address::from_str(addr)
                    .map_err(|e| {
                        VelocityError::InvalidData(format!("invalid address {addr}: {e}"))
                    })
                    .map(|unchecked: Address<NetworkUnchecked>| unchecked.assume_checked())
            })
            .collect()
    }

    fn map_rpc_error<E: std::fmt::Display>(error: E) -> VelocityError {
        VelocityError::DataSource(error.to_string())
    }

    fn classify_rpc_error(error: &bitcoincore_rpc::Error) -> RpcFailureClass {
        let message = error.to_string().to_lowercase();
        if message.contains("pruned") || message.contains("pruning") {
            return RpcFailureClass::Permanent;
        }
        if message.contains("timeout")
            || message.contains("timed out")
            || message.contains("connection")
            || message.contains("transport")
            || message.contains("broken pipe")
            || message.contains("temporarily unavailable")
        {
            return RpcFailureClass::Transient;
        }
        RpcFailureClass::Permanent
    }

    fn is_pruned_error(error: &bitcoincore_rpc::Error) -> bool {
        let message = error.to_string().to_lowercase();
        message.contains("pruned") || message.contains("pruning")
    }

    fn normalized_addresses(addresses: &[String]) -> Vec<String> {
        let mut normalized: Vec<String> = addresses.to_vec();
        normalized.sort();
        normalized.dedup();
        normalized
    }

    fn call_with_retry<T>(
        &self,
        action: &'static str,
        call: Arc<dyn Fn() -> Result<T, bitcoincore_rpc::Error> + Send + Sync>,
    ) -> Result<T, VelocityError>
    where
        T: Send + 'static,
    {
        let attempts = self.rpc_config.retry_limit.saturating_add(1);
        for attempt in 0..attempts {
            let timeout = self.rpc_config.timeout;
            let call = Arc::clone(&call);
            let (tx, rx) = std::sync::mpsc::channel();
            let started = Instant::now();
            std::thread::spawn(move || {
                let result = call();
                let _ = tx.send(result);
            });

            match rx.recv_timeout(timeout) {
                Ok(result) => match result {
                    Ok(value) => {
                        self.metrics.record_rpc_success(action, started.elapsed());
                        if attempt > 0 {
                            info!(
                                action,
                                attempt = attempt + 1,
                                "rpc call succeeded after retry"
                            );
                        }
                        return Ok(value);
                    }
                    Err(err) => {
                        if Self::is_pruned_error(&err) {
                            self.metrics.record_pruned_node(action);
                            return Err(VelocityError::InvalidData(format!(
                                "bitcoin core node is pruned or missing data: {err}"
                            )));
                        }
                        let failure_class = Self::classify_rpc_error(&err);
                        self.metrics.record_rpc_failure(action, failure_class, started.elapsed());
                        debug!(
                            action,
                            attempt = attempt + 1,
                            error = %err,
                            "rpc call failed"
                        );
                        if failure_class == RpcFailureClass::Permanent {
                            return Err(Self::map_rpc_error(err));
                        }
                        if attempt + 1 == attempts {
                            return Err(Self::map_rpc_error(err));
                        }
                    }
                },
                Err(_) => {
                    self.metrics.record_rpc_timeout(action, started.elapsed());
                    debug!(
                        action,
                        attempt = attempt + 1,
                        timeout_secs = timeout.as_secs_f64(),
                        "rpc call timed out"
                    );
                    if attempt + 1 == attempts {
                        return Err(VelocityError::DataSource(format!(
                            "rpc call timed out after {:?}: {}",
                            timeout, action
                        )));
                    }
                }
            }
        }

        Err(VelocityError::DataSource(format!(
            "rpc call failed after {} attempts: {}",
            attempts, action
        )))
    }

    fn current_height(&self) -> Result<u64, VelocityError> {
        let client = self.client.clone();
        let call = Arc::new(move || client.get_block_count());
        self.call_with_retry("get_block_count", call)
    }

    fn block_hash_for_height(&self, height: u64) -> Result<bitcoin::BlockHash, VelocityError> {
        let client = self.client.clone();
        let call = Arc::new(move || client.get_block_hash(height));
        self.call_with_retry("get_block_hash", call)
    }

    fn handle_tip_height(&self, tip_height: u64) {
        if let Ok(mut cache) = self.cache.lock() {
            if let Some(last_height) = cache.last_tip_height {
                if tip_height < last_height {
                    cache.utxos.clear();
                    cache.transactions.clear();
                    self.metrics.record_reorg(last_height, tip_height);
                    info!(
                        previous_height = last_height,
                        new_height = tip_height,
                        "chain reorg detected, cache invalidated"
                    );
                }
            }
            cache.last_tip_height = Some(tip_height);
        }
    }
}

impl ChainDataSource for BitcoinCoreChainDataSource {
    fn utxos_for_addresses(&self, addresses: &[String]) -> Result<Vec<UtxoEntry>, VelocityError> {
        let tip_height = self.current_height()?;
        self.handle_tip_height(tip_height);
        let normalized_addresses = Self::normalized_addresses(addresses);
        let cache_key = UtxoCacheKey {
            addresses: normalized_addresses.clone(),
            height: tip_height,
        };

        if let Ok(cache) = self.cache.lock() {
            if let Some(cached) = cache.utxos.get(&cache_key) {
                self.metrics.record_cache_hit("utxos");
                debug!(
                    height = tip_height,
                    address_count = cache_key.addresses.len(),
                    "utxo cache hit"
                );
                return Ok(cached.clone());
            }
        }
        self.metrics.record_cache_miss("utxos");

        info!(
            height = tip_height,
            address_count = normalized_addresses.len(),
            "fetching utxos via rpc"
        );

        let parsed_addresses = self.parse_addresses(&normalized_addresses)?;
        let scan_descriptors: Vec<String> = parsed_addresses
            .into_iter()
            .map(|addr| format!("addr({})", addr))
            .collect();

        let client = self.client.clone();
        let scan_objects: Vec<bitcoincore_rpc::json::ScanTxOutRequest> = scan_descriptors
            .iter()
            .map(|desc| bitcoincore_rpc::json::ScanTxOutRequest::Single(desc.clone()))
            .collect();
        let call = Arc::new(move || client.scan_tx_out_set_blocking(&scan_objects));
        let scan_result = self.call_with_retry("scan_tx_out_set", call)?;

        let utxos: Vec<UtxoEntry> = scan_result
            .unspents
            .into_iter()
            .map(|unspent| UtxoEntry {
                txid: unspent.txid,
                vout: unspent.vout,
                amount: unspent.amount,
                height: unspent.height,
            })
            .collect();

        if let Ok(mut cache) = self.cache.lock() {
            cache.utxos.insert(cache_key, utxos.clone());
        }

        Ok(utxos)
    }

    fn outgoing_activity_for_addresses(
        &self,
        addresses: &[String],
        start_height: u64,
        end_height: u64,
    ) -> Result<TxActivity, VelocityError> {
        if end_height < start_height {
            return Err(VelocityError::InvalidData(
                "end_height cannot be less than start_height".into(),
            ));
        }
        self.handle_tip_height(end_height);

        let normalized_addresses = Self::normalized_addresses(addresses);
        let cache_key = TxCacheKey {
            addresses: normalized_addresses.clone(),
            start_height,
            end_height,
        };

        if let Ok(cache) = self.cache.lock() {
            if let Some(cached) = cache.transactions.get(&cache_key) {
                self.metrics.record_cache_hit("transactions");
                debug!(
                    start_height,
                    end_height,
                    address_count = cache_key.addresses.len(),
                    "transaction cache hit"
                );
                return Ok(TxActivity {
                    count_outgoing: cached.count_outgoing,
                    volume_outgoing: cached.volume_outgoing,
                });
            }
        }
        self.metrics.record_cache_miss("transactions");

        info!(
            start_height,
            end_height,
            address_count = normalized_addresses.len(),
            "fetching outgoing activity via rpc"
        );

        let address_set: HashSet<String> = normalized_addresses.into_iter().collect();

        let start_hash_height = start_height.saturating_sub(1);
        let start_hash = Some(self.block_hash_for_height(start_hash_height)?);

        let client = self.client.clone();
        let call = Arc::new(move || {
            client.list_since_block(start_hash.as_ref(), None, Some(true), Some(true))
        });
        let list_result = self.call_with_retry("list_since_block", call)?;

        let mut count_outgoing: u32 = 0;
        let mut outgoing_sats: u64 = 0;

        for tx in list_result.transactions {
            if tx.detail.category != GetTransactionResultDetailCategory::Send {
                continue;
            }

            let address = match tx.detail.address.as_ref() {
                Some(addr) => addr,
                None => {
                    self.metrics.record_partial_response("list_since_block_missing_address");
                    debug!("list_since_block missing address for send tx");
                    continue;
                }
            };

            // Convert NetworkUnchecked to string via assume_checked
            let address_str = address.clone().assume_checked().to_string();
            let address_matches = address_set.contains(&address_str);

            if !address_matches {
                continue;
            }

            let block_height = match tx.info.blockheight {
                Some(block_height) => block_height as u64,
                None => {
                    self.metrics
                        .record_partial_response("list_since_block_missing_blockheight");
                    debug!("list_since_block missing blockheight for send tx");
                    continue;
                }
            };

            if block_height < start_height || block_height > end_height {
                continue;
            }

            count_outgoing = count_outgoing.saturating_add(1);
            // For send transactions, amount is negative, so we take absolute value
            let amount_sats = tx.detail.amount.to_sat().unsigned_abs();
            outgoing_sats = outgoing_sats.saturating_add(amount_sats);
        }

        let activity = TxActivity {
            count_outgoing,
            volume_outgoing: Amount::from_sat(outgoing_sats),
        };

        if let Ok(mut cache) = self.cache.lock() {
            cache.transactions.insert(
                cache_key,
                TxActivity {
                    count_outgoing: activity.count_outgoing,
                    volume_outgoing: activity.volume_outgoing,
                },
            );
        }

        Ok(activity)
    }
}

#[derive(Debug, Clone)]
pub struct RpcConfig {
    pub timeout: Duration,
    pub retry_limit: usize,
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            retry_limit: 2,
        }
    }
}

#[derive(Debug, Default)]
struct CacheState {
    utxos: HashMap<UtxoCacheKey, Vec<UtxoEntry>>,
    transactions: HashMap<TxCacheKey, TxActivity>,
    last_tip_height: Option<u64>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct UtxoCacheKey {
    addresses: Vec<String>,
    height: u64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct TxCacheKey {
    addresses: Vec<String>,
    start_height: u64,
    end_height: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RpcFailureClass {
    Transient,
    Permanent,
}

#[derive(Debug, Default)]
struct Metrics {
    rpc_success: std::sync::atomic::AtomicU64,
    rpc_failure_transient: std::sync::atomic::AtomicU64,
    rpc_failure_permanent: std::sync::atomic::AtomicU64,
    rpc_timeout: std::sync::atomic::AtomicU64,
    cache_hit: std::sync::atomic::AtomicU64,
    cache_miss: std::sync::atomic::AtomicU64,
    reorg_detected: std::sync::atomic::AtomicU64,
    partial_response: std::sync::atomic::AtomicU64,
    pruned_node: std::sync::atomic::AtomicU64,
    rpc_latency_ms_total: std::sync::atomic::AtomicU64,
    rpc_latency_ms_count: std::sync::atomic::AtomicU64,
}

impl Metrics {
    fn record_rpc_success(&self, action: &'static str, elapsed: Duration) {
        self.rpc_success
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.record_rpc_latency(elapsed);
        debug!(action, elapsed_ms = elapsed.as_millis(), "rpc success");
    }

    fn record_rpc_failure(
        &self,
        action: &'static str,
        class: RpcFailureClass,
        elapsed: Duration,
    ) {
        match class {
            RpcFailureClass::Transient => {
                self.rpc_failure_transient
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
            RpcFailureClass::Permanent => {
                self.rpc_failure_permanent
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }
        self.record_rpc_latency(elapsed);
        debug!(
            action,
            failure_class = ?class,
            elapsed_ms = elapsed.as_millis(),
            "rpc failure"
        );
    }

    fn record_rpc_timeout(&self, action: &'static str, elapsed: Duration) {
        self.rpc_timeout
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.record_rpc_latency(elapsed);
        debug!(
            action,
            elapsed_ms = elapsed.as_millis(),
            "rpc timeout"
        );
    }

    fn record_rpc_latency(&self, elapsed: Duration) {
        self.rpc_latency_ms_total.fetch_add(
            elapsed.as_millis() as u64,
            std::sync::atomic::Ordering::Relaxed,
        );
        self.rpc_latency_ms_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    fn record_cache_hit(&self, name: &'static str) {
        self.cache_hit
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        debug!(cache = name, "cache hit");
    }

    fn record_cache_miss(&self, name: &'static str) {
        self.cache_miss
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        debug!(cache = name, "cache miss");
    }

    fn record_reorg(&self, previous: u64, current: u64) {
        self.reorg_detected
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        debug!(previous, current, "reorg detected");
    }

    fn record_partial_response(&self, name: &'static str) {
        self.partial_response
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        debug!(partial = name, "partial rpc response");
    }

    fn record_pruned_node(&self, action: &'static str) {
        self.pruned_node
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        debug!(action, "pruned node detected");
    }
}
