mod db;
mod twelvedata;

use model::stock::Stock;
use sqlx::postgres::PgPool;
use std::collections::HashMap;
use twelvedata::endpoint::stocks::get_stocks;

#[tokio::main]
async fn main() {
    let pool = db::get_connection_pool().await.unwrap();

    let stocks = get_stocks(pool.to_owned()).await.unwrap();
    let existing_stock_mapping = get_stock_mapping(pool.to_owned()).await.unwrap();

    // split into existing and new stocks
    let size = stocks.len();
    let mut existing_stocks = Vec::<(Stock, Stock)>::with_capacity(size);
    let mut new_stocks = Vec::<Stock>::with_capacity(size);
    for stock in stocks {
        match existing_stock_mapping.get(&stock.symbol) {
            Some(existing_stock) => existing_stocks.push((existing_stock.clone(), stock)),
            None => new_stocks.push(stock),
        };
    }

    // Update existing stocks
    Stock::insert_many(pool.to_owned(), new_stocks)
        .await
        .unwrap();
    Stock::update_many(pool, existing_stocks).await.unwrap();
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
