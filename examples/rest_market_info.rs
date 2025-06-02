use log::info;

#[tokio::main]
async fn main() {
    // Initialize logging
    simple_logger::init_with_level(log::Level::Info).unwrap();

    // Create a new client connected to testnet
    let client = paradex::rest::Client::new(paradex::url::URL::Testnet, None)
        .await
        .unwrap();

    // Get and print all available markets
    let markets = client.markets(None).await.unwrap();
    info!("Markets: {:#?}", markets);

    let market_for_btc = client.markets(Some("BTC".to_string())).await.unwrap();
    info!("Markets BTC: {:#?}", market_for_btc);
}
