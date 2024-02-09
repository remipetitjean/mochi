use chrono::naive::{Days, NaiveDate};
use chrono::Utc;
use model::stock::Stock;
use model::stock_price::StockPrice;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use thiserror::Error;

mod db;

#[derive(Deserialize, Debug)]
struct InceptionDateModel {
    datetime: NaiveDate,
}

#[derive(Deserialize, Debug)]
struct StockPriceMetaModel {
    currency: String,
    #[serde(rename(deserialize = "mic_code"))]
    exchange: String,
}

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
    meta: StockPriceMetaModel,
    values: Vec<StockPriceValueModel>,
}

#[derive(Deserialize, Debug)]
struct ErrorModel {
    code: u16,
    message: String,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("url not found")]
    NotFound,

    #[error("resource is forbidden")]
    Forbidden,

    #[error("unhandled error: {text:}")]
    Unhandled { text: String },

    #[error("reqwest error {code:?}: {text:}")]
    Reqwest { code: u16, text: String },
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

async fn json_from_api<T>(url: &str) -> Result<T, ApiError>
where
    T: for<'a> Deserialize<'a> + std::fmt::Debug,
{
    // add api key
    let api_key = "16ebf3860688468b9cdab89899669b30";
    let url_with_api_key = format!("{}&apikey={}", url, api_key);
    println!("{}", url_with_api_key);

    // request data
    let response = reqwest::get(url_with_api_key).await.unwrap();
    let code = response.status();
    let text = response.text().await.unwrap();
    println!("GET {} -> {}", url, code);
    if !code.is_success() {
        return Err(ApiError::Reqwest {
            code: code.into(),
            text: text.to_string(),
        });
    }

    // try deserializing into T
    let data: Result<T, serde_json::error::Error> = serde_json::from_str(&text);
    match data {
        Ok(res) => return Ok(res),
        Err(_) => (),
    };

    // try deserializing into ApiError
    let err: Result<ErrorModel, serde_json::error::Error> = serde_json::from_str(&text);
    match err {
        Ok(_) => return Err(ApiError::Forbidden),
        Err(err) => {
            return Err(ApiError::Unhandled {
                text: err.to_string(),
            })
        }
    };
}

#[tokio::main]
async fn main() {
    let pool = db::get_connection_pool().await.unwrap();

    // retrieve companies
    let _stocks = Stock::select(pool.to_owned()).await.unwrap();

    let _ = retrieve_and_store_stock_price_since_inception(pool.to_owned(), "AAPL").await;

    //let start_date = NaiveDate::from_ymd(1983, 9, 23);
    //let end_date = NaiveDate::from_ymd(1983, 9, 24);
    //let end_date = NaiveDate::from_ymd(1983, 9, 25);
    //let symbol = "AAPL";
    //retrieve_and_store_stock_price(pool, symbol, start_date, end_date).await.expect("youpi");
    // let _ = retrieve_and_store_since_inception(pool.to_owned(), "7974").await;
    //for stock in stocks {
    //    let _ = retrieve_and_store_since_inception(pool.to_owned(), &stock.symbol).await;
    //    break;
    //}

    // let stock_prices = StockPrice::select(pool)
    //     .await
    //     .unwrap();
    // println!("{:?}", stock_prices);

    //let input_stock_prices = read_input_stock_prices(pool.to_owned()).await;

    // Update existing stock_prices
    // StockPrice::insert_many(pool.to_owned(), new_stock_prices)
    //     .await
    //     .unwrap();
    // StockPrice::update_many(pool, existing_stock_prices).await.unwrap();
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
    let inception_date_model = json_from_api::<InceptionDateModel>(&inception_date_url)
        .await
        .unwrap();
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
    let stock_prices = json_from_api::<StockPriceModel>(&stock_prices_url)
        .await
        .unwrap();

    println!("Converting into db model");
    let stock_price_meta = stock_prices.meta;
    let exchange = &stock_price_meta.exchange;
    let currency = &stock_price_meta.currency;
    let stock_price_values = stock_prices.values;
    let mut db_stock_prices: Vec<StockPrice> = Vec::with_capacity(stock_price_values.len());
    for stock_price in stock_price_values {
        let db_stock_price = StockPrice {
            symbol: symbol.to_string(),
            exchange: exchange.clone(),
            currency: currency.clone(),
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
    StockPrice::insert_many(pool, db_stock_prices)
        .await
        .unwrap();

    Ok(())
}

async fn retrieve_and_store_stock_price_since_inception(
    pool: PgPool,
    symbol: &str,
) -> Result<(), ApiError> {
    println!("Retrieving and storing prices for {}", symbol);

    // get inception date
    let mut start_date = get_inception_date(symbol).await.unwrap();
    let mut end_date = add_days(start_date, 10);

    // stock prices
    let today = Utc::now().date_naive();
    while end_date <= today {
        retrieve_and_store_stock_price(pool.to_owned(), symbol, start_date, end_date)
            .await
            .unwrap();
        start_date = end_date;
        end_date = add_days(start_date, 5000);
    }

    retrieve_and_store_stock_price(pool.to_owned(), symbol, start_date, today)
        .await
        .unwrap();

    Ok(())
}

// Retrieve inception date
//let inception_date = retrieve_earliest_timestamp(symbol).await;

//println!("Inception date for {}: {:?}", symbol, inception_date);

//    Ok(())
//}
