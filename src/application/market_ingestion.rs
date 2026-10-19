use std::str::FromStr;
use std::sync::Arc;

use rust_decimal::Decimal;
use serde::Deserialize;
use thiserror::Error;

use crate::application::ports::{
    MarketDataSource, MarketDataSourceError, MarketTradePublisher, PublishError,
};
use crate::domain::trade::MarketTrade;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct IngestionStats {
    pub connection_attempts: u64,
    pub messages_received: u64,
    pub trades_published: u64,
    pub malformed_messages: u64,
}

#[derive(Debug, Error)]
pub enum IngestionError {
    #[error("Market data source unavailable: {0}")]
    SourceUnavailable(String),

    #[error(transparent)]
    Publish(#[from] PublishError),
}

pub struct MarketIngestionService {
    source: Arc<dyn MarketDataSource>,
    publisher: Arc<dyn MarketTradePublisher>,
    max_reconnects: u32,
}

#[derive(Debug, Deserialize)]
struct RawTrade {
    symbol: String,
    price: String,
    quantity: String,
    timestamp_ms: u64,
}

impl MarketIngestionService {
    pub fn new(
        source: Arc<dyn MarketDataSource>,
        publisher: Arc<dyn MarketTradePublisher>,
        max_reconnects: u32,
    ) -> Self {
        Self {
            source,
            publisher,
            max_reconnects,
        }
    }

    pub async fn run(&self) -> Result<IngestionStats, IngestionError> {
        let mut stats = IngestionStats::default();
        let mut reconnects = 0;

        loop {
            stats.connection_attempts += 1;

            let mut receiver = match self.source.connect().await {
                Ok(receiver) => receiver,
                Err(error) if reconnects < self.max_reconnects => {
                    reconnects += 1;
                    let _ = error;
                    continue;
                }
                Err(error) => {
                    return Err(IngestionError::SourceUnavailable(error.to_string()));
                }
            };

            while let Some(payload) = receiver.recv().await {
                stats.messages_received += 1;

                let Some(trade) = parse_trade(&payload) else {
                    stats.malformed_messages += 1;
                    continue;
                };

                self.publisher.publish(trade).await?;
                stats.trades_published += 1;
            }

            // A closed stream means the connection ended.
            // Retry only while reconnects remain available.
            if reconnects >= self.max_reconnects {
                break;
            }

            reconnects += 1;
        }

        Ok(stats)
    }
}

fn parse_trade(payload: &str) -> Option<MarketTrade> {
    let raw: RawTrade = serde_json::from_str(payload).ok()?;

    let price = Decimal::from_str(&raw.price).ok()?;
    let quantity = Decimal::from_str(&raw.quantity).ok()?;

    MarketTrade::new(raw.symbol, price, quantity, raw.timestamp_ms).ok()
}

#[cfg(test)]
#[path = "market_ingestion_tests.rs"]
mod market_ingestion_tests;
