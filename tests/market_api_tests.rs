use crypto_market_platform_rust::build_app;
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let response = build_app()
        .oneshot(
            axum::http::Request::builder()
                .uri("/health")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json, serde_json::json!({ "status": "ok" }));
}

#[tokio::test]
async fn list_markets_returns_seeded_markets() {
    let response = build_app()
        .oneshot(
            axum::http::Request::builder()
                .uri("/markets")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let markets: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();

    assert_eq!(markets.len(), 2);
    assert!(markets.iter().any(|m| m["symbol"] == "BTCUSDT"));
    assert!(markets.iter().any(|m| m["symbol"] == "ETHUSDT"));
}

#[tokio::test]
async fn get_market_returns_specific_market() {
    let response = build_app()
        .oneshot(
            axum::http::Request::builder()
                .uri("/markets/BTCUSDT")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let market: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(market["symbol"], "BTCUSDT");
    assert_eq!(market["base_asset"], "BTC");
    assert_eq!(market["quote_asset"], "USDT");
}

#[tokio::test]
async fn get_market_returns_not_found_for_nonexistent_market() {
    let response = build_app()
        .oneshot(
            axum::http::Request::builder()
                .uri("/markets/NONEXISTENT")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["error"], "Market not found: NONEXISTENT");
}
