use model::prelude::{
    CurrencyController, DatabaseConnection, Exchange, ExchangeController, NewStockExchange, Stock,
    StockController, StockExchangeController, StockType,
};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct ExchangeApi {
    data: Vec<Exchange>,
}

pub fn deserialize_type<'de, D>(deserializer: D) -> Result<StockType, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    Ok(StockType::from_string(&buf))
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct StockExchangeApi {
    #[serde(rename(deserialize = "symbol"))]
    pub id: String,
    pub name: String,
    pub country: String,
    #[serde(deserialize_with = "deserialize_type")]
    pub r#type: StockType,
    #[serde(rename(deserialize = "exchange"))]
    pub exchange_id: String,
    #[serde(rename(deserialize = "currency"))]
    pub currency_id: Option<String>,
    pub mic_code: String,
}

#[derive(Debug, Deserialize)]
struct StockApi {
    data: Vec<StockExchangeApi>,
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct StockExchangeTuple {
    stock_id: String,
    exchange_id: String,
}

pub async fn update_exchange(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.twelvedata.com/exchanges").await?;
    let exchange_api: ExchangeApi = resp.json().await?;
    let exchanges = exchange_api.data;
    ExchangeController::insert_many(&db, exchanges).await?;
    Ok(())
}

pub async fn update_stock(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.twelvedata.com/stocks").await?;
    let stock_api: StockApi = resp.json().await?;
    let exchange_stocks_api = stock_api.data;

    // stocks
    let mut stock_map: HashMap<String, Stock> = HashMap::new();
    for stock in exchange_stocks_api.clone() {
        if !stock_map.contains_key(&stock.id) {
            stock_map.insert(
                stock.id.clone(),
                Stock {
                    id: stock.id,
                    name: stock.name.clone(),
                    country: stock.country.clone(),
                    r#type: stock.r#type.clone(),
                },
            );
        }
    }
    let stocks: Vec<Stock> = stock_map
        .values()
        .into_iter()
        .map(|x| x.to_owned())
        .collect();
    StockController::insert_many(&db, stocks).await?;

    // stock_exchange
    let mut stock_exchange_map: HashMap<StockExchangeTuple, NewStockExchange> = HashMap::new();
    let currency_map: HashMap<String, String> = CurrencyController::hashmap_name_to_id(&db).await?;
    for stock in exchange_stocks_api {
        let key = StockExchangeTuple {
            stock_id: stock.id.clone(),
            exchange_id: stock.exchange_id.clone(),
        };
        if !stock_exchange_map.contains_key(&key) {
            let currency_id: Option<String> = match currency_map.get(&stock.name) {
                Some(currency_id) => Some(currency_id.clone()),
                None => None,
            };
            stock_exchange_map.insert(
                key,
                NewStockExchange {
                    stock_id: stock.id,
                    exchange_id: stock.exchange_id,
                    currency_id: stock.currency_id,
                    mic_code: stock.mic_code,
                },
            );
        }
        let stock_exchanges: Vec<NewStockExchange> = stock_exchange_map
            .values()
            .into_iter()
            .map(|x| x.to_owned())
            .collect();
        StockExchangeController::insert_many(&db, stock_exchanges).await?;
    }

    Ok(())
}
