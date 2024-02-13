use super::super::api::{json_from_endpoint, ApiError};
use model::country::Country;
use model::exchange::Exchange;
use serde::Deserialize;
use sqlx::postgres::PgPool;

const ENDPOINT: &str = "exchanges";

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct ExchangeModel {
    code: String,
    name: String,
    country: String,
    timezone: String,
}

#[derive(Deserialize, Debug)]
struct Model {
    data: Vec<ExchangeModel>,
}

async fn get() -> Result<Model, ApiError> {
    json_from_endpoint::<Model>(ENDPOINT, false).await
}

pub async fn get_exchanges(pool: PgPool) -> Result<Vec<Exchange>, ApiError> {
    let exchange_models = get().await?.data;

    let country_hashmap_by_name = Country::get_hashmap_by_name(pool).await.unwrap();

    let mut exchanges = Vec::with_capacity(exchange_models.len());
    for exchange_model in exchange_models.iter() {
        let country_name = &exchange_model.country;
        let country_code = &country_hashmap_by_name[country_name].code;
        let exchange = Exchange {
            code: exchange_model.code.to_string(),
            name: exchange_model.name.to_string(),
            country: country_code.to_string(),
            timezone: exchange_model.timezone.to_string(),
        };
        exchanges.push(exchange);
    }

    Ok(exchanges)
}
