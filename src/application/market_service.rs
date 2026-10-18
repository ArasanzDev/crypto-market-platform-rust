use std::sync::Arc;

use thiserror::Error;

use crate::{application::ports::MarketRepository, domain::market::Market};

#[derive(Debug, Error)]
pub enum MarketServiceError {
    #[error("Market not found: {0}")]
    MarketNotFound(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),
}

pub struct MarketService {
    repository: Arc<dyn MarketRepository>,
}

impl MarketService {
    pub fn new(repository: Arc<dyn MarketRepository>) -> Self {
        Self { repository }
    }

    pub async fn list_markets(&self) -> Result<Vec<Market>, MarketServiceError> {
        self.repository
            .find_all()
            .await
            .map_err(MarketServiceError::RepositoryError)
    }

    pub async fn get_market(&self, symbol: &str) -> Result<Market, MarketServiceError> {
        match self.repository.find_by_symbol(symbol).await {
            Ok(Some(market)) => Ok(market),
            Ok(None) => Err(MarketServiceError::MarketNotFound(symbol.to_string())),
            Err(e) => Err(MarketServiceError::RepositoryError(e)),
        }
    }
}
