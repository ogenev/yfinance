use reqwest::Client;
use serde_json::Value;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub chart: Chart,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub result: Vec<Result>,
    pub error: Value,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub meta: Meta,
    pub timestamp: Vec<i64>,
    pub indicators: Indicators,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub currency: String,
    pub symbol: String,
    pub exchange_name: String,
    pub instrument_type: String,
    pub first_trade_date: i64,
    pub regular_market_time: i64,
    pub gmtoffset: i64,
    pub timezone: String,
    pub exchange_timezone_name: String,
    pub regular_market_price: f64,
    pub chart_previous_close: f64,
    pub previous_close: f64,
    pub scale: i64,
    pub price_hint: i64,
    pub current_trading_period: CurrentTradingPeriod,
    pub trading_periods: Vec<Vec<TradingPeriod>>,
    pub data_granularity: String,
    pub range: String,
    pub valid_ranges: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrentTradingPeriod {
    pub pre: Pre,
    pub regular: Regular,
    pub post: Post,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pre {
    pub timezone: String,
    pub start: i64,
    pub end: i64,
    pub gmtoffset: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Regular {
    pub timezone: String,
    pub start: i64,
    pub end: i64,
    pub gmtoffset: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub timezone: String,
    pub start: i64,
    pub end: i64,
    pub gmtoffset: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradingPeriod {
    pub timezone: String,
    pub start: i64,
    pub end: i64,
    pub gmtoffset: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Indicators {
    pub quote: Vec<Quote>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub low: f64,
    pub volume: f64,
    pub close: f64,
    pub open: f64,
    pub high: f64,
}


pub struct Downloader<'a> {
    base_url: &'a str,
    ticker_list: Vec<&'a str>
}

impl<'a> Downloader<'a> {
    pub fn new(ticker_list: Vec<&'a str>) -> Self {
        Downloader {
            base_url: "https://query1.finance.yahoo.com/v8/finance/chart/",
            ticker_list
        }
    }

    #[tokio::main]
    pub async fn run(&self) {
        let mut tasks:Vec<Root> = vec![];
        let client = Client::new();
        for ticker in &self.ticker_list {
            let url = self.base_url.to_owned() + ticker;
            tasks.push({
               client.get(&url).send().await.unwrap().json::<Root>().await.unwrap()
            });
        }
       println!("{:?}", tasks);
    }
}


pub fn test() {
    let downloader = Downloader::new(vec!["MSFT", "TSLA"]);
    downloader.run();
}