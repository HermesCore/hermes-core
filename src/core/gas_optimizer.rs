// core/gas_optimizer.rs

use ethers::{prelude::*, utils::parse_units};
use std::sync::Arc;

pub struct GasOptimizer {
    provider: Arc<Provider<Http>>,
    wallet: LocalWallet,
}

impl GasOptimizer {
    pub fn new(provider_url: &str, private_key: &str) -> Self {
        let provider = Provider::<Http>::try_from(provider_url).unwrap();
        let wallet = private_key.parse().unwrap();
        Self {
            provider: Arc::new(provider),
            wallet,
        }
    }

    pub async fn optimize_gas(&self, transaction: TransactionRequest) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
        // Here we would implement logic to calculate gas prices and optimize the transaction
        let gas_price = self.provider.get_gas_price().await?;
        
        let tx = transaction.gas_price(gas_price).gas(100_000);
        
        let client = Arc::new(SignerMiddleware::new(self.provider.clone(), self.wallet.clone()));
        let tx_hash = client.send_transaction(tx, None).await?;

        Ok(tx_hash)
    }
}
