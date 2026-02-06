use crate::utxo_scoring::UtxoEntry;
use crate::velocity_analyzer::{ChainDataSource, TxActivity, VelocityError};
use bitcoin::util::amount::Amount;
use bitcoin::Address;
use bitcoincore_rpc::json::{ListSinceBlockCategory, ScanTxOutRequest};
use bitcoincore_rpc::{Client, RpcApi};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{debug, info};

/// Bitcoin Core-backed chain data source.
#[derive(Debug, Clone)]
pub struct BitcoinCoreChainDataSource {
    client: Client,
    rpc_config: RpcConfig,
    cache: Arc<Mutex<CacheState>>,
}

impl BitcoinCoreChainDataSource {
    pub fn new(client: Client) -> Self {
        Self::new_with_config(client, RpcConfig::default())
    }

    pub fn new_with_config(client: Client, rpc_config: RpcConfig) -> Self {
        Self {
            client,
            rpc_config,
            cache: Arc::new(Mutex::new(CacheState::default())),
        }
    }

    fn parse_addresses(&self, addresses: &[String]) -> Result<Vec<Address>, VelocityError> {
        addresses
            .iter()
            .map(|addr| {
                Address::from_str(addr).map_err(|e| {
                    VelocityError::InvalidData(format!("invalid address {addr}: {e}"))
                })
            })
            .collect()
    }

    fn map_rpc_error<E: std::fmt::Display>(error: E) -> VelocityError {
        VelocityError::DataSource(error.to_string())
    }

    fn normalized_addresses(addresses: &[String]) -> Vec<String> {
        let mut normalized: Vec<String> = addresses.iter().cloned().collect();
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
            std::thread::spawn(move || {
                let result = call();
                let _ = tx.send(result);
            });

            match rx.recv_timeout(timeout) {
                Ok(result) => match result {
                    Ok(value) => {
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
                        debug!(
                            action,
                            attempt = attempt + 1,
                            error = %err,
                            "rpc call failed"
                        );
                        if attempt + 1 == attempts {
                            return Err(Self::map_rpc_error(err));
                        }
                    }
                },
                Err(_) => {
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
            .map(|height| height as u64)
    }

    fn block_hash_for_height(&self, height: u64) -> Result<bitcoin::BlockHash, VelocityError> {
        let client = self.client.clone();
        let call = Arc::new(move || client.get_block_hash(height));
        self.call_with_retry("get_block_hash", call)
    }
}

impl ChainDataSource for BitcoinCoreChainDataSource {
    fn utxos_for_addresses(&self, addresses: &[String]) -> Result<Vec<UtxoEntry>, VelocityError> {
        let tip_height = self.current_height()?;
        let normalized_addresses = Self::normalized_addresses(addresses);
        let cache_key = UtxoCacheKey {
            addresses: normalized_addresses.clone(),
            height: tip_height,
        };

        if let Ok(cache) = self.cache.lock() {
            if let Some(cached) = cache.utxos.get(&cache_key) {
                debug!(
                    height = tip_height,
                    address_count = cache_key.addresses.len(),
                    "utxo cache hit"
                );
                return Ok(cached.clone());
            }
        }

        info!(
            height = tip_height,
            address_count = normalized_addresses.len(),
            "fetching utxos via rpc"
        );

        let parsed_addresses = self.parse_addresses(&normalized_addresses)?;
        let scan_objects: Vec<ScanTxOutRequest> = parsed_addresses
            .into_iter()
            .map(ScanTxOutRequest::Addr)
            .collect();

        let client = self.client.clone();
        let call = Arc::new(move || client.scan_tx_out_set_blocking(&scan_objects));
        let scan_result = self.call_with_retry("scan_tx_out_set", call)?;

        let utxos = scan_result
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
        let normalized_addresses = Self::normalized_addresses(addresses);
        let cache_key = TxCacheKey {
            addresses: normalized_addresses.clone(),
            start_height,
            end_height,
        };

        if let Ok(cache) = self.cache.lock() {
            if let Some(cached) = cache.transactions.get(&cache_key) {
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
            if tx.category != ListSinceBlockCategory::Send {
                continue;
            }

            let address_matches = tx
                .address
                .as_ref()
                .map(|addr| address_set.contains(&addr.to_string()))
                .unwrap_or(false);

            if !address_matches {
                continue;
            }

            if let Some(block_height) = tx.blockheight {
                if block_height < start_height || block_height > end_height {
                    continue;
                }
            }

            count_outgoing = count_outgoing.saturating_add(1);
            outgoing_sats = outgoing_sats.saturating_add(tx.amount.to_sat());
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
