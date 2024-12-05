use reqwest;
use serde::Deserialize;
use tokio::time::{self, Duration};

#[derive(Deserialize, Debug)]
struct TickerPrice {
    symbol: String,
    price: String,
}



#[tokio::main]
async fn main() {
    let api_url = "https://api.binance.com/api/v3/ticker/price";
    let trading_symbol = "BTCUSDT";
    let mut interval = time::interval(Duration::from_secs(2));
    println!("Fetching price of {} every 2 seconds", trading_symbol);

    loop {
        interval.tick().await;
        match fetch_price(api_url, trading_symbol).await {
            Ok(price) => println!("{} price: {}", trading_symbol, price),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

async fn fetch_price(api_url: &str, symbol: &str) -> Result<String, Box<dyn std::error::Error>>{
    let url = format!("{}?symbol={}", api_url, symbol);
    let response = reqwest::get(&url).await?.json::<TickerPrice>().await?;
    Ok(response.price)
}