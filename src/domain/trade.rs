use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketTrade {
    symbol: String,
    price: Decimal,
    quantity: Decimal,
    timestamp: u64,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TradeError {
    #[error("Invalid symbol: {0}")]
    InvalidSymbol(String),

    #[error("Trade price must be positive: {0}")]
    InvalidPrice(Decimal),

    #[error("Trade quantity must be positive: {0}")]
    InvalidQuantity(Decimal),
}

impl MarketTrade {
    pub fn new(
        symbol: impl Into<String>,
        price: Decimal,
        quantity: Decimal,
        timestamp: u64,
    ) -> Result<Self, TradeError> {
        let market_symbol = symbol.into();

        if market_symbol.is_empty() {
            return Err(TradeError::InvalidSymbol(market_symbol));
        }
        if price <= Decimal::ZERO {
            return Err(TradeError::InvalidPrice(price));
        }
        if quantity <= Decimal::ZERO {
            return Err(TradeError::InvalidQuantity(quantity));
        }

        Ok(Self {
            symbol: market_symbol,
            price,
            quantity,
            timestamp,
        })
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn price(&self) -> Decimal {
        self.price
    }

    pub fn quantity(&self) -> Decimal {
        self.quantity
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

#[cfg(test)]
#[path = "trade_test.rs"]
mod market_test;
