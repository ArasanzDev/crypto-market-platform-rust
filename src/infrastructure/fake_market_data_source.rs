use std::collections::VecDeque;

use async_trait::async_trait;
use tokio::sync::{Mutex, mpsc};

use crate::{
    application::ports::{
        MarketDataSource, MarketDataSourceError, MarketTradePublisher, PublishError,
    },
    domain::trade::MarketTrade,
};

pub struct FakeMarketDataSource {
    batches: Mutex<VecDeque<Vec<String>>>,
}

impl FakeMarketDataSource {
    pub fn new(batches: Vec<Vec<String>>) -> Self {
        Self {
            batches: Mutex::new(batches.into()),
        }
    }
}

#[async_trait]
impl MarketDataSource for FakeMarketDataSource {
    async fn connect(&self) -> Result<mpsc::Receiver<String>, MarketDataSourceError> {
        let batch = self
            .batches
            .lock()
            .await
            .pop_front()
            .ok_or_else(|| MarketDataSourceError::new("No more batches"))?;

        let (sender, receiver) = mpsc::channel(batch.len().max(1));

        for message in batch {
            sender
                .send(message)
                .await
                .map_err(|e| MarketDataSourceError::new(e.to_string()))?;
        }

        drop(sender);

        Ok(receiver)
    }
}

#[derive(Default)]
pub struct InMemoyrMarketTradePublisher {
    trades: Mutex<Vec<MarketTrade>>,
}

impl InMemoyrMarketTradePublisher {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn get_trades(&self) -> Vec<MarketTrade> {
        self.trades.lock().await.clone()
    }
}

#[async_trait]
impl MarketTradePublisher for InMemoyrMarketTradePublisher {
    async fn publish(&self, trade: MarketTrade) -> Result<(), PublishError> {
        self.trades.lock().await.push(trade);
        Ok(())
    }
}
