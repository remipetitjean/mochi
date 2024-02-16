mod db;
mod twelvedata;

use chrono::naive::{Days, NaiveDate};
use chrono::Utc;
use model::stock::Stock;
use model::stock_price::StockPrice;
use sqlx::postgres::PgPool;
use std::collections::HashMap;
use thiserror::Error;
use tokio::time::{sleep, Duration};
use twelvedata::api::ApiError;
use twelvedata::endpoint::{earliest_timestamp, time_series};

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("unhandled error: {text:}")]
    Unhandled { text: String },
}

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("API error")]
    ApiError(#[from] ApiError),

    #[error("database error")]
    DatabaseError(#[from] DatabaseError),
}

#[tokio::main]
async fn main() {
    let pool = db::get_connection_pool().await.unwrap();

    // retrieve companies
    let stocks = Stock::select_with_plans(pool.to_owned()).await.unwrap();
    let max_eod_hashmap = StockPrice::get_eod_hashmap(pool.to_owned()).await.unwrap();
    let end_date = twelvedata_today();

    // storing prices
    for chunk in stocks.chunks(17) {
        for stock in chunk {
            let symbol = stock.symbol.to_string();
            let start_date = get_stock_start_date(&symbol, max_eod_hashmap.to_owned()).await;
            let _ =
                chunk_and_store_stock_prices(pool.to_owned(), &symbol, start_date, end_date, 5000)
                    .await;
        }
        sleep(Duration::from_millis(60000)).await;
    }
}

fn twelvedata_today() -> NaiveDate {
    Utc::now().date_naive()
    //let today = Utc::now().date_naive();
    //today.checked_add_days(Days::new(1)).expect("should have a value")
}

fn add_days(date: NaiveDate, num_days: u64, max_date: NaiveDate) -> NaiveDate {
    let date = date
        .clone()
        .checked_add_days(Days::new(num_days))
        .expect("should have a value");

    match date > max_date {
        true => max_date,
        false => date,
    }
}

async fn get_stock_start_date(
    symbol: &str,
    max_eod_hashmap: HashMap<String, NaiveDate>,
) -> NaiveDate {
    let max_eod = max_eod_hashmap.get(symbol);
    match max_eod {
        Some(max_price_date) => max_price_date
            .clone()
            .checked_add_days(Days::new(1))
            .expect("should not be empty"),
        None => earliest_timestamp::get_inception_date(&symbol)
            .await
            .expect("should have something"),
    }
}

async fn store_stock_price(
    pool: PgPool,
    symbol: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<(), LoaderError> {
    let stock_prices = time_series::get_time_series(symbol, start_date, end_date).await?;
    let _ = StockPrice::insert_many(pool, stock_prices).await;
    Ok(())
}

async fn chunk_and_store_stock_prices(
    pool: PgPool,
    symbol: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
    chunk_size: u64,
) -> Result<(), LoaderError> {
    println!(
        "Storing prices for {} [{:?} - {:?}]",
        symbol, start_date, end_date
    );
    if start_date == end_date {
        return Ok(());
    }

    let mut chunk_start_date = start_date;
    let mut chunk_end_date = add_days(chunk_start_date, chunk_size, end_date);

    // stock prices
    loop {
        store_stock_price(pool.to_owned(), symbol, chunk_start_date, chunk_end_date).await?;
        chunk_start_date = chunk_end_date;
        chunk_end_date = add_days(chunk_start_date, chunk_size, end_date);
        if chunk_start_date == end_date {
            break;
        }
    }

    Ok(())
}
