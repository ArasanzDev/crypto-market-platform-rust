use async_trait::async_trait;

use crate::domain::market::Market;

#[async_trait]
pub trait MarketRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Market>, String>;
    async fn find_by_symbol(&self, symbol: &str) -> Result<Option<Market>, String>;
}
