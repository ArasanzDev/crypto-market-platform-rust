use std::env;

use crypto_market_platform_rust::{
    application::ports::MarketTradePublisher, domain::trade::MarketTrade,
    infrastructure::kafka_trade_publisher::KafkaMarketTradePublisher,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bootstrap_servers =
        env::var("KAFKA_BOOTSTRAP_SERVERS").unwrap_or_else(|_| "localhost:9092".to_string());

    let publisher = KafkaMarketTradePublisher::new(&bootstrap_servers)?;

    let trade = MarketTrade::new(
        "BTCUSDT",
        rust_decimal::Decimal::new(300000, 2),
        rust_decimal::Decimal::new(1, 2),
        1797000000000,
    )?;

    publisher.publish(trade).await?;

    println!("Trade published successfully to Kafka topic.");

    Ok(())
}
