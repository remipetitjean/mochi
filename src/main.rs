//https://api.twelvedata.com/exchanges?format=csv
//https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
use model::prelude::Exchange;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExchangeApi {
    data: Vec<Exchange>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.twelvedata.com/exchanges").await?;
    let exchange_api: ExchangeApi = resp.json().await?;
    //let exchanges = resp.json().await?;
    println!("{:#?}", exchange_api);
    //println!("{:#?}", resp);
    Ok(())
}
