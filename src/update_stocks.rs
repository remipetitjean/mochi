use model::country::Country;
use model::stock::Stock;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use std::collections::HashMap;

mod db;

#[derive(Clone, Debug, Deserialize)]
pub struct Stockx {
    pub symbol: String,
    pub name: String,
    pub currency: String,
    #[serde(rename(deserialize = "mic_code"))]
    pub exchange: String,
    pub country: String,
    #[serde(rename(deserialize = "type"))]
    pub stock_type: String,
}

#[derive(Deserialize)]
struct ApiStock {
    data: Vec<Stock>,
}

#[tokio::main]
async fn main() {
    let pool = db::get_connection_pool().await.unwrap();

    let input_stocks = read_input_stocks(pool.to_owned()).await;
    println!("{:?}", input_stocks);

    // retrieve existing stocks
    let stock_mapping = get_stock_mapping(pool.to_owned()).await.unwrap();

    // split into existing and new stocks
    let size = input_stocks.len();
    let mut existing_stocks = Vec::<(Stock, Stock)>::with_capacity(size);
    let mut new_stocks = Vec::<Stock>::with_capacity(size);
    for input_stock in input_stocks {
        match stock_mapping.get(&input_stock.symbol) {
            Some(existing_stock) => existing_stocks.push((existing_stock.clone(), input_stock)),
            None => new_stocks.push(input_stock),
        };
    }

    // Update existing stocks
    Stock::insert_many(pool.to_owned(), new_stocks)
        .await
        .unwrap();
    Stock::update_many(pool, existing_stocks).await.unwrap();
}

async fn read_input_stocks(pool: PgPool) -> Vec<Stock> {
    // country overwite
    let mut country_by_symbol_map: HashMap<String, String> = HashMap::new();
    country_by_symbol_map.insert("FREETR".to_string(), "DK".to_string());
    country_by_symbol_map.insert("WUSH".to_string(), "RU".to_string());

    // stocks
    let url = "https://api.twelvedata.com/stocks";
    let response = reqwest::get(url).await.unwrap();

    if response.status() != reqwest::StatusCode::OK {
        panic!("KO");
    }

    let api_stock = response
        .json::<ApiStock>()
        .await
        .expect("Cannot deserialize to Vec<Stock>");
    let mut stocks = api_stock.data;

    // country map
    let country_by_name_map = Country::to_name_hash_map(pool.clone())
        .await
        .expect("Must get country map");

    // update country to country code
    for stock in stocks.iter_mut() {
        let country_name = &stock.country;

        let country_code: String = match country_name.as_str() {
            "" => {
                let symbol = &stock.symbol;
                String::from(&country_by_symbol_map[symbol])
            }
            _ => String::from(&country_by_name_map[country_name].code),
        };

        stock.country = country_code;
    }

    stocks
}

async fn get_stock_mapping(pool: PgPool) -> Result<HashMap<String, Stock>, sqlx::Error> {
    let stocks = Stock::select(pool).await?;
    let mut stock_mapping = HashMap::<String, Stock>::new();

    for stock in stocks {
        let symbol = stock.symbol.clone();
        stock_mapping.insert(symbol, stock);
    }

    Ok(stock_mapping)
}
