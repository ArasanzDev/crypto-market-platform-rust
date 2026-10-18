use std::collections::HashMap;

use async_trait::async_trait;

use crate::{application::ports::MarketRepository, domain::market::Market};

pub struct InMemoryMarketRepository {
    markets: HashMap<String, Market>,
}

impl InMemoryMarketRepository {
    pub fn new() -> Self {
        let markets = [
            Market::new("BTCUSDT", "BTC", "USDT").unwrap(),
            Market::new("ETHUSDT", "ETH", "USDT").unwrap(),
        ]
        .into_iter()
        .map(|market| (market.symbol().to_owned(), market))
        .collect();

        Self { markets }
    }
}

impl Default for InMemoryMarketRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MarketRepository for InMemoryMarketRepository {
    async fn find_all(&self) -> Result<Vec<Market>, String> {
        Ok(self.markets.values().cloned().collect())
    }

    async fn find_by_symbol(&self, symbol: &str) -> Result<Option<Market>, String> {
        Ok(self.markets.get(symbol).cloned())
    }
}
