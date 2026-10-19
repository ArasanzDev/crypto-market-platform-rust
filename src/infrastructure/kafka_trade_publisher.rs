use async_trait::async_trait;
use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};
use serde::Serialize;

use crate::{
    application::ports::{MarketTradePublisher, PublishError},
    domain::trade::MarketTrade,
};

pub const MARKET_TRADE_TOPIC: &str = "market.trades";

#[derive(Debug, Serialize)]
struct MarketTradeEvent {
    schema_version: u32,
    symbol: String,
    price: String,
    quantity: String,
    timestamp_ms: u64,
}

impl From<MarketTrade> for MarketTradeEvent {
    fn from(trade: MarketTrade) -> Self {
        Self {
            schema_version: 1,
            symbol: trade.symbol().to_string(),
            price: trade.price().to_string(),
            quantity: trade.quantity().to_string(),
            timestamp_ms: trade.timestamp(),
        }
    }
}

pub struct KafkaMarketTradePublisher {
    producer: FutureProducer,
    topic: String,
}

impl KafkaMarketTradePublisher {
    pub fn new(bootstrap_servers: &str) -> Result<Self, PublishError> {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", bootstrap_servers)
            .set("message.timeout.ms", "5000")
            .set("acks", "all")
            .set("enable.idempotence", "true")
            .create()
            .map_err(|e| PublishError(e.to_string()))?;

        Ok(Self {
            producer,
            topic: MARKET_TRADE_TOPIC.to_owned(),
        })
    }
}

#[async_trait]
impl MarketTradePublisher for KafkaMarketTradePublisher {
    async fn publish(&self, trade: MarketTrade) -> Result<(), PublishError> {
        let event = MarketTradeEvent::from(trade);
        let payload = serde_json::to_string(&event).map_err(|e| PublishError(e.to_string()))?;

        let key = event.symbol.clone();

        self.producer
            .send(
                FutureRecord::to(&self.topic).payload(&payload).key(&key),
                Timeout::After(std::time::Duration::from_secs(5)),
            )
            .await
            .map_err(|(e, _)| PublishError(e.to_string()))?;

        Ok(())
    }
}
