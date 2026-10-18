use std::sync::Arc;

use crate::{
    api::market_routes::{AppState, router},
    application::market_service::MarketService,
    infrastructure::in_memory_market_repository::InMemoryMarketRepository,
};

pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

pub fn build_app() -> axum::Router {
    let repository = Arc::new(InMemoryMarketRepository::new());
    let service = Arc::new(MarketService::new(repository));

    router(AppState {
        market_service: service,
    })
}
