use crate::utxo_scoring::UtxoEntry;
use crate::velocity_analyzer::{ChainDataSource, TxActivity, VelocityError};
use bitcoin::util::amount::Amount;
use bitcoin::Address;
use bitcoincore_rpc::json::{ListSinceBlockCategory, ScanTxOutRequest};
use bitcoincore_rpc::{Client, RpcApi};
use std::collections::HashSet;
use std::str::FromStr;

/// Bitcoin Core-backed chain data source.
#[derive(Debug, Clone)]
pub struct BitcoinCoreChainDataSource {
    client: Client,
}

impl BitcoinCoreChainDataSource {
    pub fn new(client: Client) -> Self {
        Self { client }
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
}

impl ChainDataSource for BitcoinCoreChainDataSource {
    fn utxos_for_addresses(&self, addresses: &[String]) -> Result<Vec<UtxoEntry>, VelocityError> {
        let parsed_addresses = self.parse_addresses(addresses)?;
        let scan_objects: Vec<ScanTxOutRequest> = parsed_addresses
            .into_iter()
            .map(ScanTxOutRequest::Addr)
            .collect();

        let scan_result = self
            .client
            .scan_tx_out_set_blocking(&scan_objects)
            .map_err(Self::map_rpc_error)?;

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

        Ok(utxos)
    }

    fn outgoing_activity_for_addresses(
        &self,
        addresses: &[String],
        start_height: u64,
        end_height: u64,
    ) -> Result<TxActivity, VelocityError> {
        let address_set: HashSet<String> =
            addresses.iter().map(|address| address.to_string()).collect();

        let start_hash = if start_height == 0 {
            None
        } else {
            Some(
                self.client
                    .get_block_hash(start_height.saturating_sub(1))
                    .map_err(Self::map_rpc_error)?,
            )
        };

        let list_result = self
            .client
            .list_since_block(
                start_hash.as_ref(),
                None,
                Some(true),
                Some(true),
            )
            .map_err(Self::map_rpc_error)?;

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

        Ok(TxActivity {
            count_outgoing,
            volume_outgoing: Amount::from_sat(outgoing_sats),
        })
    }
}
