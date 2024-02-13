mod db;
mod twelvedata;

use model::exchange::Exchange;
use sqlx::postgres::PgPool;
use std::collections::HashMap;
use twelvedata::endpoint::exchanges::get_exchanges;

#[tokio::main]
async fn main() {
    let pool = db::get_connection_pool().await.unwrap();

    let exchanges = get_exchanges(pool.to_owned()).await.unwrap();

    // retrieve existing exchanges
    let existing_exchange_mapping = get_existing_exchange_mapping(pool.to_owned())
        .await
        .unwrap();

    // split into existing and new exchanges
    let size = exchanges.len();
    let mut existing_exchanges = Vec::<(Exchange, Exchange)>::with_capacity(size);
    let mut new_exchanges = Vec::<Exchange>::with_capacity(size);
    for input_exchange in exchanges {
        match existing_exchange_mapping.get(&input_exchange.code) {
            Some(existing_exchange) => {
                existing_exchanges.push((existing_exchange.clone(), input_exchange))
            }
            None => new_exchanges.push(input_exchange),
        };
    }

    // Update existing exchanges
    Exchange::insert_many(pool.to_owned(), new_exchanges)
        .await
        .unwrap();
    Exchange::update_many(pool, existing_exchanges)
        .await
        .unwrap();
}

async fn get_existing_exchange_mapping(
    pool: PgPool,
) -> Result<HashMap<String, Exchange>, sqlx::Error> {
    let exchanges = Exchange::select(pool).await?;
    let mut existing_exchange_mapping = HashMap::<String, Exchange>::new();

    for exchange in exchanges {
        let code = exchange.code.clone();
        existing_exchange_mapping.insert(code, exchange);
    }

    Ok(existing_exchange_mapping)
}
