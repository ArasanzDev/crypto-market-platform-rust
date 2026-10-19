use std::fmt;

use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::domain::{market::Market, trade::MarketTrade};

#[async_trait]
pub trait MarketRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Market>, String>;
    async fn find_by_symbol(&self, symbol: &str) -> Result<Option<Market>, String>;
}

#[derive(Debug)]
pub struct MarketDataSourceError {
    message: String,
}

impl MarketDataSourceError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for MarketDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MarketDataSourceError {}

#[async_trait]
pub trait MarketDataSource: Send + Sync {
    async fn connect(&self) -> Result<mpsc::Receiver<String>, MarketDataSourceError>;
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to publish market trade: {0}")]
pub struct PublishError(pub String);

#[async_trait]
pub trait MarketTradePublisher: Send + Sync {
    async fn publish(&self, trade: MarketTrade) -> Result<(), PublishError>;
}
