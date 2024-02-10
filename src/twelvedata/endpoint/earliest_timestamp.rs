use super::super::api::{json_from_endpoint, ApiError};
use chrono::naive::NaiveDate;
use serde::Deserialize;

const ENDPOINT: &str = "earliest_timestamp";

#[derive(Deserialize, Debug)]
pub struct InceptionDateModel {
    datetime: NaiveDate,
}

pub async fn get(symbol: &str) -> Result<InceptionDateModel, ApiError> {
    println!("Retrieving inception date for {}", symbol);
    let endpoint_with_params = format!("{}?symbol={}&interval=1day", ENDPOINT, symbol);
    let model = json_from_endpoint::<InceptionDateModel>(&endpoint_with_params).await?;

    Ok(model)
}

pub async fn get_inception_date(symbol: &str) -> Result<NaiveDate, ApiError> {
    let inception_date_model = get(symbol).await?;
    Ok(inception_date_model.datetime)
}
