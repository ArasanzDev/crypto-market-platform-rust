use std::sync::Arc;

use crate::{
    application::market_ingestion::MarketIngestionService,
    infrastructure::fake_market_data_source::{FakeMarketDataSource, InMemoyrMarketTradePublisher},
};

fn valid_trade(symbol: &str, price: &str) -> String {
    format!(
        r#"{{"symbol":"{symbol}","price":"{price}","quantity":"0.01","timestamp_ms":1797000000000}}"#
    )
}

#[tokio::test]
async fn publishes_valid_trades() {
    let source = Arc::new(FakeMarketDataSource::new(vec![vec![
        valid_trade("BTCUSDT", "30000.0"),
        valid_trade("ETHUSDT", "2000.0"),
    ]]));

    let publisher = Arc::new(InMemoyrMarketTradePublisher::new());

    let service = MarketIngestionService::new(source, publisher.clone(), 0);

    let stats = service.run().await.unwrap();
    let trades = publisher.get_trades().await;

    assert_eq!(stats.connection_attempts, 1);
    assert_eq!(stats.messages_received, 2);
    assert_eq!(stats.trades_published, 2);
    assert_eq!(stats.malformed_messages, 0);
    assert_eq!(trades.len(), 2);
    assert_eq!(trades[0].symbol(), "BTCUSDT");
}

#[tokio::test]
async fn skips_malformed_trades() {
    let source = Arc::new(FakeMarketDataSource::new(vec![vec![
        valid_trade("BTCUSDT", "30000.0"),
        r#"{"symbol":"ETHUSDT","price":"2000.0","quantity":"0.01"}"#.to_string(), // Missing timestamp_ms
    ]]));

    let publisher = Arc::new(InMemoyrMarketTradePublisher::new());

    let service = MarketIngestionService::new(source, publisher.clone(), 0);

    let stats = service.run().await.unwrap();
    let trades = publisher.get_trades().await;

    assert_eq!(stats.connection_attempts, 1);
    assert_eq!(stats.messages_received, 2);
    assert_eq!(stats.trades_published, 1);
    assert_eq!(stats.malformed_messages, 1);
    assert_eq!(trades.len(), 1);
    assert_eq!(trades[0].symbol(), "BTCUSDT");
}

#[tokio::test]
async fn reconnects_when_stream_terminates() {
    let source = Arc::new(FakeMarketDataSource::new(vec![
        vec![valid_trade("BTCUSDT", "30000.0")],
        vec![valid_trade("ETHUSDT", "2000.0")],
    ]));

    let publisher = Arc::new(InMemoyrMarketTradePublisher::new());

    let service = MarketIngestionService::new(source, publisher.clone(), 1);

    let stats = service.run().await.unwrap();
    let trades = publisher.get_trades().await;

    assert_eq!(stats.connection_attempts, 2);
    assert_eq!(stats.messages_received, 2);
    assert_eq!(stats.trades_published, 2);
    assert_eq!(stats.malformed_messages, 0);
    assert_eq!(trades.len(), 2);
}

#[tokio::test]
async fn stops_after_max_reconnects() {
    let source = Arc::new(FakeMarketDataSource::new(vec![
        vec![valid_trade("BTCUSDT", "30000.0")],
        vec![valid_trade("ETHUSDT", "2000.0")],
    ]));

    let publisher = Arc::new(InMemoyrMarketTradePublisher::new());

    let service = MarketIngestionService::new(source, publisher.clone(), 0);

    let stats = service.run().await.unwrap();
    let trades = publisher.get_trades().await;

    assert_eq!(stats.connection_attempts, 1);
    assert_eq!(stats.messages_received, 1);
    assert_eq!(stats.trades_published, 1);
    assert_eq!(stats.malformed_messages, 0);
    assert_eq!(trades.len(), 1);
}
