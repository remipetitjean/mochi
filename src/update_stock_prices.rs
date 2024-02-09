mod db;
mod td_api;

use chrono::naive::{Days, NaiveDate};
use chrono::Utc;
use model::stock::Stock;
use model::stock_price::StockPrice;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use td_api::{json_from_api, ApiError};
use thiserror::Error;

#[derive(Deserialize, Debug)]
struct InceptionDateModel {
    datetime: NaiveDate,
}

//#[derive(Deserialize, Debug)]
//struct StockPriceMetaModel {
//    currency: String,
//    #[serde(rename(deserialize = "mic_code"))]
//    exchange: String,
//}

#[derive(Deserialize, Debug)]
struct StockPriceValueModel {
    #[serde(rename(deserialize = "datetime"))]
    eod: NaiveDate,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String,
}

#[derive(Deserialize, Debug)]
struct StockPriceModel {
    //    meta: StockPriceMetaModel,
    values: Vec<StockPriceValueModel>,
}

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

    for stock in stocks {
        let _ =
            retrieve_and_store_stock_price_since_inception(pool.to_owned(), &stock.symbol).await;
    }

    // let start_date = NaiveDate::from_ymd(1983, 9, 23);
    // let end_date = NaiveDate::from_ymd(1983, 9, 25);
    // let symbol = "000001";
    // let _ = retrieve_and_store_stock_price_since_inception(pool.to_owned(), symbol).await;
}

// https://api.twelvedata.com/time_series?symbol=AAPL&interval=1day&start_date=1980-12-12&outputsize=5000&apikey=16ebf3860688468b9cdab89899669b30

fn add_days(date: NaiveDate, num_days: u64) -> NaiveDate {
    date.clone()
        .checked_add_days(Days::new(num_days))
        .expect("should have a value")
}

async fn get_inception_date(symbol: &str) -> Result<NaiveDate, ApiError> {
    println!("Retriving inception date for {}", symbol);
    let inception_date_endpoint = "https://api.twelvedata.com/earliest_timestamp";
    let inception_date_url = format!(
        "{}?symbol={}&interval=1day",
        inception_date_endpoint, symbol
    );
    let inception_date_model = json_from_api::<InceptionDateModel>(&inception_date_url).await?;

    Ok(inception_date_model.datetime)
}

async fn retrieve_and_store_stock_price(
    pool: PgPool,
    symbol: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<(), LoaderError> {
    println!(
        "Retrieving prices for {} [{} - {}]",
        symbol, start_date, end_date
    );
    let stock_prices_endpoint = "https://api.twelvedata.com/time_series";
    let stock_prices_url = format!(
        "{}?symbol={}&interval=1day&start_date={}&end_date={}",
        stock_prices_endpoint, symbol, start_date, end_date
    );
    let stock_prices = json_from_api::<StockPriceModel>(&stock_prices_url).await?;

    println!("Converting into db model");
    let stock_price_values = stock_prices.values;
    let mut db_stock_prices: Vec<StockPrice> = Vec::with_capacity(stock_price_values.len());
    for stock_price in stock_price_values {
        let db_stock_price = StockPrice {
            symbol: symbol.to_string(),
            eod: stock_price.eod,
            open: stock_price.open.parse::<f64>().unwrap(),
            high: stock_price.high.parse::<f64>().unwrap(),
            low: stock_price.low.parse::<f64>().unwrap(),
            close: stock_price.close.parse::<f64>().unwrap(),
            volume: stock_price.volume.parse::<i64>().unwrap(),
        };
        db_stock_prices.push(db_stock_price);
    }

    println!("Storing in the db");
    let _ = StockPrice::insert_many(pool, db_stock_prices).await;

    Ok(())
}

async fn retrieve_and_store_stock_price_since_inception(
    pool: PgPool,
    symbol: &str,
) -> Result<(), LoaderError> {
    println!("Retrieving and storing prices for {}", symbol);

    // get inception date
    let mut start_date = get_inception_date(symbol).await?;
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
