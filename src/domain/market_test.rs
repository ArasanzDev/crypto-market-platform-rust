use crate::domain::market::{Market, MarketError};

#[test]
fn create_valid_market() {
    let market = Market::new("BTCUSDT", "BTC", "USDT").unwrap();
    assert_eq!(market.symbol(), "BTCUSDT");
    assert_eq!(market.base_asset(), "BTC");
    assert_eq!(market.quote_asset(), "USDT");
}

#[test]
fn rejects_empty_symbol() {
    let result = Market::new("", "BTC", "USDT");
    assert!(matches!(result, Err(MarketError::EmptySymbol)));
}

#[test]
fn rejects_empty_base_asset() {
    let result = Market::new("BTCUSDT", "", "USDT");
    assert!(matches!(result, Err(MarketError::EmptyBaseAsset)));
}

#[test]
fn rejects_invalid_symbol() {
    let result = Market::new("btcusdt", "BTC", "USDT");
    assert!(matches!(result, Err(MarketError::InvalidSymbol)));
}
