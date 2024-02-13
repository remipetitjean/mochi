use super::super::api::{json_from_endpoint, ApiError};
use chrono::naive::NaiveDate;
use model::stock_price::StockPrice;
use serde::Deserialize;

const ENDPOINT: &str = "time_series";

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ValueModel {
    #[serde(rename(deserialize = "datetime"))]
    eod: NaiveDate,
    open: String,
    high: String,
    low: String,
    close: String,
    volume: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct MetaModel {
    symbol: String,
    interval: String,
    currency: String,
    #[serde(rename(deserialize = "mic_code"))]
    exchange: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct TimeSeriesModel {
    meta: MetaModel,
    values: Vec<ValueModel>,
}

pub async fn get_time_series(
    symbol: &str,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<StockPrice>, ApiError> {
    println!(
        "Retrieving time series for {} [{} - {}]",
        symbol, start_date, end_date
    );
    let endpoint_with_params = format!(
        "{}?symbol={}&interval=1day&start_date={}&end_date={}",
        ENDPOINT, symbol, start_date, end_date,
    );
    let model = json_from_endpoint::<TimeSeriesModel>(&endpoint_with_params, true).await?;

    let values = model.values;
    let mut stock_prices: Vec<StockPrice> = Vec::with_capacity(values.len());
    for value in values {
        let stock_price = StockPrice {
            symbol: symbol.to_string(),
            eod: value.eod,
            open: value.open.parse::<f64>().unwrap(),
            high: value.high.parse::<f64>().unwrap(),
            low: value.low.parse::<f64>().unwrap(),
            close: value.close.parse::<f64>().unwrap(),
            volume: value.volume.parse::<i64>().unwrap(),
        };
        stock_prices.push(stock_price);
    }
    Ok(stock_prices)
}
