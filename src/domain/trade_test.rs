use std::time::SystemTime;

use rust_decimal::Decimal;

use crate::domain::trade::{MarketTrade, TradeError};

#[test]
fn create_valid_trade() {
    let trade = MarketTrade::new(
        "BTCUSDT",
        Decimal::new(50000, 0),
        Decimal::new(1, 0),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    )
    .unwrap();

    assert_eq!(trade.symbol(), "BTCUSDT");
    assert_eq!(trade.price(), Decimal::new(50000, 0));
    assert_eq!(trade.quantity(), Decimal::new(1, 0));
}

#[test]
fn rejects_invalid_symbol() {
    let result = MarketTrade::new(
        "",
        Decimal::new(50000, 0),
        Decimal::new(1, 0),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    assert!(matches!(result, Err(TradeError::InvalidSymbol(_))));
}

#[test]
fn rejects_non_positive_price() {
    let result = MarketTrade::new(
        "BTCUSDT",
        Decimal::new(0, 0),
        Decimal::new(1, 0),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    assert!(matches!(result, Err(TradeError::InvalidPrice(_))));
}

#[test]
fn rejects_non_positive_quantity() {
    let result = MarketTrade::new(
        "BTCUSDT",
        Decimal::new(50000, 0),
        Decimal::new(0, 0),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    assert!(matches!(result, Err(TradeError::InvalidQuantity(_))));
}
