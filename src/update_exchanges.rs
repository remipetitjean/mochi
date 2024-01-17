use model::exchange::Exchange;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use std::collections::HashMap;

mod db;

#[derive(Deserialize)]
struct ApiExchange {
    data: Vec<Exchange>,
}

#[tokio::main]
async fn main() {
    let input_exchanges = read_input_exchanges().await;
    println!("exchanges = {:?}", input_exchanges);

    //let pool = db::get_connection_pool().await.unwrap();

    //// retrieve existing exchanges
    //let exchange_mapping = get_exchange_mapping(pool.to_owned()).await.unwrap();

    //// split into existing and new exchanges
    //let size = input_exchanges.len();
    //let mut existing_exchanges = Vec::<(Exchange, Exchange)>::with_capacity(size);
    //let mut new_exchanges = Vec::<Exchange>::with_capacity(size);
    //for input_exchange in input_exchanges {
    //    match exchange_mapping.get(&input_exchange.code) {
    //        Some(existing_exchange) => {
    //            existing_exchanges.push((existing_exchange.clone(), input_exchange))
    //        }
    //        None => new_exchanges.push(input_exchange),
    //    };
    //}

    //// Update existing exchanges
    //Exchange::insert_many(pool.to_owned(), new_exchanges)
    //    .await
    //    .unwrap();
    //Exchange::update_many(pool, existing_exchanges)
    //    .await
    //    .unwrap();
}

async fn read_input_exchanges() -> Vec<Exchange> {
    let url = "https://api.twelvedata.com/exchanges";
    let response = reqwest::get(url).await.unwrap();

    if response.status() != reqwest::StatusCode::OK {
        panic!("KO");
    }

    let api_exchange = response
        .json::<ApiExchange>()
        .await
        .expect("Cannot deserialize to Vec<Exchange>");
    api_exchange.data

    //let mut reader = Reader::from_path("seed/exchange.csv").unwrap();
    //let iter = reader.deserialize();
    //iter.into_iter().flatten().collect()
    //vec![Exchange {
    //    code: "XX".to_string(),
    //    name: "YY".to_string(),
    //    country_code: "US".to_string(),
    //    timezone_code: "Europe/Paris".to_string(),
    //}]
}

async fn get_exchange_mapping(pool: PgPool) -> Result<HashMap<String, Exchange>, sqlx::Error> {
    let exchanges = Exchange::select(pool).await?;
    let mut exchange_mapping = HashMap::<String, Exchange>::new();

    for exchange in exchanges {
        let code = exchange.code.clone();
        exchange_mapping.insert(code, exchange);
    }

    Ok(exchange_mapping)
}
