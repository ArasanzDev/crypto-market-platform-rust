#[cfg(test)]
#[path = "market_test.rs"]
mod market_test;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Market {
    symbol: String,
    base_asset: String,
    quote_asset: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MarketError {
    #[error("Market symbol cannot be empty")]
    EmptySymbol,

    #[error("Base asset cannot be empty")]
    EmptyBaseAsset,

    #[error("Market symbol must contain uppercase ASCII letters or digits")]
    InvalidSymbol,
}

impl Market {
    pub fn new(
        symbol: impl Into<String>,
        base_asset: impl Into<String>,
        quote_asset: impl Into<String>,
    ) -> Result<Self, MarketError> {
        let symbol = symbol.into();
        let base_asset = base_asset.into();
        let quote_asset = quote_asset.into();

        if symbol.trim().is_empty() {
            return Err(MarketError::EmptySymbol);
        }
        if base_asset.trim().is_empty() {
            return Err(MarketError::EmptyBaseAsset);
        }
        if !symbol
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        {
            return Err(MarketError::InvalidSymbol);
        }

        Ok(Self {
            symbol,
            base_asset,
            quote_asset,
        })
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn base_asset(&self) -> &str {
        &self.base_asset
    }

    pub fn quote_asset(&self) -> &str {
        &self.quote_asset
    }
}
