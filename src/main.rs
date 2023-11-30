use std::fmt;

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use tokio;

#[derive(Serialize, Deserialize, Debug)]
struct KrakenResponse {
    error: Vec<String>,
    result: KrakenResult,
}

#[derive(Serialize, Deserialize, Debug)]
struct KrakenResult {
    #[serde(rename = "XXBTZUSD")]
    xxbtzusd: TickerInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct TickerInfo {
    a: Vec<String>,
    b: Vec<String>,
    #[serde(deserialize_with = "deserialize_first_element")]
    h: String,
    #[serde(deserialize_with = "deserialize_first_element")]
    l: String,
}
impl fmt::Display for TickerInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_string = format!(
            "Asks: {:?}\nBids: {:?}\nLow: {}\nHigh: {}",
            self.a, self.b, self.l, self.h
        );
        write!(f, "{:?}", formatted_string)
    }
}

fn deserialize_first_element<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v: Vec<String> = Deserialize::deserialize(deserializer)?;
    v.into_iter()
        .next()
        .ok_or_else(|| serde::de::Error::custom("Array format is wrong"))
}

// get the bid/ask values of kraken OB
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the client. You could add headers like User-Agent if needed.
    let client = reqwest::Client::new();

    // Make a request to the Coinbase API for the BTC-USD order book.
    let response_text = client
        .get("https://api.kraken.com/0/public/Ticker")
        .query(&[("pair", "XBTUSD")])
        .send()
        .await?
        .text()
        .await?;

    let parsed: KrakenResponse = serde_json::from_str(&response_text)?;
    let parsed_values = &parsed.result.xxbtzusd;
    println!("{:}", parsed_values);

    Ok(())
}
