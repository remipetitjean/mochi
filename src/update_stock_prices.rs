mod db;
mod twelvedata;

use chrono::naive::{Days, NaiveDate};
use chrono::Utc;
use model::stock::Stock;
use model::stock_price::StockPrice;
use sqlx::postgres::PgPool;
use thiserror::Error;
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
    let stocks = Stock::select(pool.to_owned()).await.unwrap();

    // storing prices
    for stock in stocks {
        let _ =
            retrieve_and_store_stock_price_since_inception(pool.to_owned(), &stock.symbol).await;
    }
}

fn add_days(date: NaiveDate, num_days: u64) -> NaiveDate {
    date.clone()
        .checked_add_days(Days::new(num_days))
        .expect("should have a value")
}

async fn retrieve_and_store_stock_price(
    pool: PgPool,
    symbol: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<(), LoaderError> {
    let stock_prices = time_series::get_time_series(symbol, start_date, end_date).await?;
    let _ = StockPrice::insert_many(pool, stock_prices).await;
    Ok(())
}

async fn retrieve_and_store_stock_price_since_inception(
    pool: PgPool,
    symbol: &str,
) -> Result<(), LoaderError> {
    println!("Retrieving and storing prices for {}", symbol);

    // get inception date
    let mut start_date = earliest_timestamp::get_inception_date(symbol).await?;
    let mut end_date = add_days(start_date, 10);

    // stock prices
    let today = Utc::now().date_naive();
    while end_date <= today {
        retrieve_and_store_stock_price(pool.to_owned(), symbol, start_date, end_date).await?;
        start_date = end_date;
        end_date = add_days(start_date, 5000);
    }

    retrieve_and_store_stock_price(pool.to_owned(), symbol, start_date, today)
        .await
        .unwrap();

    Ok(())
}
