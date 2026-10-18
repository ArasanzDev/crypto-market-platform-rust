#[tokio::main]
async fn main() {
    let app = crypto_market_platform_rust::build_app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    println!("Crypto Market Platform is running on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start the server");
}
