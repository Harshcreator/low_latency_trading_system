use reqwest;
use serde::Deserialize;
use tokio::time::{self, Duration};
use std::collections::VecDeque;

#[derive(Deserialize, Debug)]
struct TickerPrice {
    symbol: String,
    price: String,
}



#[tokio::main]
async fn main() {
    let api_url = "https://api.binance.com/api/v3/ticker/price";
    let trading_symbol = "BTCUSDT";

    let mut price_history: VecDeque<f64> = VecDeque::new();
    let max_history = 10;


    let mut interval = time::interval(Duration::from_secs(2));

    println!("Fetching price of {} every 2 seconds", trading_symbol);

    loop {
        interval.tick().await;
        match fetch_price(api_url, trading_symbol).await {
            Ok(price) => {
                println!("{} price: {}", trading_symbol, price);
                update_price_history(&mut price_history, price.parse().unwrap(), max_history);
                
                if let Some(ma) = calculate_moving_average(&price_history) {
                    println!("Moving Average (last {} prices): {:.2}", price_history.len(), ma);
                }
            }
            Err(e) => eprintln!("Failed to fetch price: {}", e),
        }
    }
}

async fn fetch_price(api_url: &str, symbol: &str) -> Result<String, Box<dyn std::error::Error>>{
    let url = format!("{}?symbol={}", api_url, symbol);
    let response = reqwest::get(&url).await?.json::<TickerPrice>().await?;
    Ok(response.price)
}

fn update_price_history(history: &mut VecDeque<f64>, price: f64, max_history: usize) {
    history.push_back(price);
    if history.len() > max_history {
        history.pop_front();
    }
}

fn calculate_moving_average(history: &VecDeque<f64>) -> Option<f64> {
    if history.is_empty() {
        return None;
    }
    let sum: f64 = history.iter().sum();
    Some(sum / history.len() as f64)
}