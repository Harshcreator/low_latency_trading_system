use dashmap::mapref::entry;
use reqwest;
use serde::Deserialize;
use tokio::time::{self, Duration};
use std::collections::VecDeque;
use chrono::Utc;

#[derive(Deserialize, Debug)]
struct TickerPrice {
    symbol: String,
    price: String,
}

#[derive(Debug)]
struct Trade {
    action: String,
    price: f64,
    timestamp: String,
}

const STOP_LOSS_PERCENT: f64 = 2.00;
const TAKE_PROFIT_PERCENT: f64 = 3.0;

#[tokio::main]
async fn main() {
    let api_url = "https://api.binance.com/api/v3/ticker/price";
    let trading_symbol = "BTCUSDT";

    let mut price_history: VecDeque<f64> = VecDeque::new();
    let mut trade_log: Vec<Trade> = Vec::new();
    let max_history = 10;

    let mut is_in_position = false;
    let mut entry_price: Option<f64> = None;

    let mut interval = time::interval(Duration::from_secs(2));

    println!("Fetching price of {} every 2 seconds", trading_symbol);

    loop {
        interval.tick().await;

        match fetch_price(api_url, trading_symbol).await {
            Ok(price) => {
                println!("{} price: {:.8}", trading_symbol, price);

                update_price_history(&mut price_history, price.parse().unwrap(), max_history);
                
                if let Some(ma) = calculate_moving_average(&price_history) {
                    println!("Moving Average (last {} prices): {:.2}", price_history.len(), ma);

                    let (signal, new_position_status) = trading_signal(price.parse().unwrap(), ma, is_in_position, entry_price);

                    if signal == "BUY" {
                        entry_price = Some(price.parse().unwrap()); 
                        log_trade(&mut trade_log, &signal, price.parse().unwrap());
                    } else if signal == "SELL" {
                        entry_price = None;
                        log_trade(&mut trade_log, &signal, price.parse().unwrap());
                    }

                    is_in_position = new_position_status;
                    println!("Trading Signal: {}", signal);
                }
            }
            Err(e) => eprintln!("Failed to fetch price: {}", e),
        }

        if trade_log.len() >= 10{
            break;
        }
    }

    evaluate_performance(&trade_log);
}

async fn fetch_price(api_url: &str, symbol: &str) -> Result<String, Box<dyn std::error::Error>>{
    let url = format!("{}?symbol={}", api_url, symbol);
    let response = reqwest::get(&url).await?.json::<TickerPrice>().await?;
    Ok(response.price.parse::<f64>().unwrap().to_string())
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

fn trading_signal(
    current_price: f64, 
    moving_average: f64, 
    is_in_position: bool,
    entry_price: Option<f64>,
) -> (String, bool) {
    if is_in_position {
        if let Some(price) = entry_price {
            let stop_loss_price = price * (1.0 - STOP_LOSS_PERCENT / 100.0);
            let take_profit_price = price * (1.0 + TAKE_PROFIT_PERCENT / 100.0);

            if current_price <= stop_loss_price {
                return ("SELL".to_string(), false); 
            } else if current_price >= take_profit_price {
                return ("SELL".to_string(), false);
            }
        }
        return ("HOLD".to_string(), is_in_position);
    } else if current_price < moving_average {
        return ("BUY".to_string(), true); 
    } else {
        return ("HOLD".to_string(), is_in_position);
    }
}

fn log_trade(trade_log: &mut Vec<Trade>, action: &str, price: f64) {
    let trade = Trade {
        action: action.to_string(),
        price: price,
        timestamp: Utc::now().to_rfc3339(),
    };
    println!("Logged trade: {:?}", trade);
    trade_log.push(trade);
}

fn evaluate_performance(trade_log: &Vec<Trade>) {
    let mut total_profit = 0.0;
    let mut last_buy_price = None;

    for trade in trade_log {
        match trade.action.as_str() {
            "BUY" => {
                last_buy_price = Some(trade.price);
            }
            "SELL" => {
                if let Some(buy_price) = last_buy_price {
                    let profit = trade.price - buy_price;
                    total_profit += profit;
                    println!(
                        "Trade: Sold at {:.2}, Bought at {:.2}, Profit: {:.2}",
                        trade.price, buy_price, profit
                    );
                    last_buy_price = None;
                }
            }
            _ => {}
        }
    }
    println!("Total Profit: {:.2}", total_profit);
}