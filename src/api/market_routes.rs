use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use serde::Serialize;

use crate::{
    application::market_service::{MarketService, MarketServiceError},
    domain::market::Market,
};

#[derive(Clone)]
pub struct AppState {
    pub market_service: Arc<MarketService>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/markets", get(list_markets))
        .route("/markets/{symbol}", get(get_market))
        .with_state(state)
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn list_markets(State(state): State<AppState>) -> Result<Json<Vec<Market>>, ApiError> {
    let markets = state
        .market_service
        .list_markets()
        .await
        .map_err(ApiError::from)?;

    Ok(Json(markets))
}

async fn get_market(
    State(state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<Market>, ApiError> {
    let market = state
        .market_service
        .get_market(&symbol)
        .await
        .map_err(ApiError::from)?;

    Ok(Json(market))
}

struct ApiError(MarketServiceError);

impl From<MarketServiceError> for ApiError {
    fn from(err: MarketServiceError) -> Self {
        Self(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self.0 {
            MarketServiceError::MarketNotFound(symbol) => (
                StatusCode::NOT_FOUND,
                format!("Market not found: {}", symbol),
            ),
            MarketServiceError::RepositoryError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Repository error: {}", err),
            ),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}
