use ethers::prelude::*;
use ethers::middleware::flashbots::*;
use crate::execution::bundle::{BundleExecutor, estimate_profit};
use crate::{Order, ExecutionResult, OrderSide};
use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH, SystemTime};
use url::Url;

pub struct FlashbotsExecutor {
    pub provider: Arc<Provider<Http>>,
    pub searcher: LocalWallet,
    pub relay_url: String,
}

impl FlashbotsExecutor {
    pub fn new(rpc_url: &str, relay_url: &str, key: &str) -> Self {
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();
        let signer: LocalWallet = key.parse().unwrap();
        Self {
            provider: Arc::new(provider),
            searcher: signer,
            relay_url: relay_url.to_string(),
        }
    }
}

impl BundleExecutor for FlashbotsExecutor {
    fn simulate_profit(&self, orders: &[Order]) -> f64 {
        estimate_profit(orders)
    }

    fn execute_bundle(&self, orders: &[Order]) -> ExecutionResult {
        let fb_signer = self.searcher.clone();
        let flashbots_provider = FlashbotsMiddleware::new(
            self.provider.clone(),
            fb_signer.clone(),
            Url::parse(&self.relay_url).unwrap(),
        );

        let txs: Vec<TypedTransaction> = orders.iter().map(|order| {
            let tx = TransactionRequest::new()
                .to(order.pool_address.parse::<Address>().unwrap())
                .gas(21000)
                .data(Bytes::from_static(&[0xde, 0xad, 0xbe, 0xef]));
            TypedTransaction::Legacy(tx)
        }).collect();

        let signed_txs: Vec<Bytes> = txs.iter()
            .map(|tx| fb_signer.sign_transaction(tx).unwrap())
            .collect();

        let latest_block = self.provider.get_block_number().unwrap();
        let target_block = latest_block + 1u64;

        let bundle = BundleRequest::new()
            .set_block(target_block.as_u64())
            .push_txs(signed_txs.clone());

        let sim_result = flashbots_provider.simulate_bundle(&bundle);
        if let Ok(sim) = sim_result {
            if sim.first_revert.is_some() {
                return ExecutionResult {
                    success: false,
                    gas_used: 0,
                    tx_hash: None,
                    notes: Some("Simulation failed: revert".into()),
                };
            }
        } else {
            return ExecutionResult {
                success: false,
                gas_used: 0,
                tx_hash: None,
                notes: Some("Simulation error".into()),
            };
        }

        let send_result = flashbots_provider.send_bundle(&bundle);
        match send_result {
            Ok(pending) => ExecutionResult {
                success: true,
                gas_used: (orders.len() * 21000) as u64,
                tx_hash: Some(format!("{:?}", pending.bundle_hash())),
                notes: Some("Bundle submitted successfully".into()),
            },
            Err(err) => ExecutionResult {
                success: false,
                gas_used: 0,
                tx_hash: None,
                notes: Some(format!("Submission error: {:?}", err)),
            },
        }
    }
}
