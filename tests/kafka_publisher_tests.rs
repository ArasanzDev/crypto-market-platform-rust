use std::time::Duration;

use crypto_market_platform_rust::{
    application::ports::MarketTradePublisher,
    domain::trade::MarketTrade,
    infrastructure::kafka_trade_publisher::{KafkaMarketTradePublisher, MARKET_TRADE_TOPIC},
};
use rdkafka::{
    config::ClientConfig,
    consumer::{Consumer, StreamConsumer},
    message::Message,
};
use rust_decimal::Decimal;
use testcontainers_modules::{kafka::Kafka, testcontainers::runners::SyncRunner};
use tokio::runtime::Runtime;

#[test]
fn publishes_trade_event_to_kafka() {
    let container = Kafka::default()
        .start()
        .expect("Failed to start Kafka container");

    let host = container.get_host().expect("Failed to get Kafka host");
    let port = container
        .get_host_port_ipv4(9093)
        .expect("Failed to get Kafka port");

    let bootstrap_servers = format!("{}:{}", host, port);

    let runtime = Runtime::new().unwrap();

    runtime.block_on(async {
        let publisher = KafkaMarketTradePublisher::new(&bootstrap_servers)
            .expect("Failed to create KafkaMarketTradePublisher");

        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &bootstrap_servers)
            .set("group.id", "market-publisher-test")
            .set("auto.offset.reset", "earliest")
            .set("enable.auto.commit", "false")
            .create()
            .expect("Failed to create Kafka consumer");

        consumer
            .subscribe(&[MARKET_TRADE_TOPIC])
            .expect("Failed to subscribe to topic");

        let trade = MarketTrade::new(
            "BTCUSDT",
            Decimal::new(300000, 2),
            Decimal::new(1, 2),
            1797000000000,
        )
        .unwrap();

        publisher
            .publish(trade)
            .await
            .expect("Failed to publish trade");

        let message = tokio::time::timeout(Duration::from_secs(5), consumer.recv())
            .await
            .expect("Failed to receive message")
            .expect("No message received");

        assert_eq!(message.topic(), MARKET_TRADE_TOPIC);

        let key = message.key().expect("Message has no key");

        assert_eq!(key, b"BTCUSDT");

        let payload = message.payload().expect("Message has no payload");

        let event: serde_json::Value =
            serde_json::from_slice(payload).expect("Failed to deserialize payload");

        assert_eq!(event["schema_version"], 1);
        assert_eq!(event["symbol"], "BTCUSDT");
        assert_eq!(event["price"], "3000.00");
        assert_eq!(event["quantity"], "0.01");
        assert_eq!(event["timestamp_ms"], 1797000000000_u64);
    });
}
