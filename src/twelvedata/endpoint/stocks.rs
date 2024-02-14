use super::super::api::{json_from_endpoint, ApiError};
use model::country::Country;
use model::stock::{Stock, StockType};
use model::stock_td_plan::{StockTdPlan, PlanType, GlobalType};
use serde::Deserialize;
use sqlx::postgres::PgPool;
use std::collections::HashSet;

const ENDPOINT: &str = "stocks";

#[allow(dead_code)]
#[derive(Deserialize)]
struct AccessModel {
    global: GlobalType,
    plan: PlanType,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct DataModel {
    symbol: String,
    name: String,
    currency: String,
    #[serde(rename(deserialize = "mic_code"))]
    exchange: String,
    country: String,
    #[serde(rename(deserialize = "type"))]
    stock_type: StockType,
    access: Option<AccessModel>,
}

#[derive(Deserialize)]
struct Model {
    data: Vec<DataModel>,
}

async fn get(show_plan: bool) -> Result<Model, ApiError> {
    println!("Retrieving stocks");
    let endpoint_with_params = format!("{}?show_plan={}", ENDPOINT, show_plan);
    json_from_endpoint::<Model>(&endpoint_with_params, true).await
}

pub async fn get_stocks(pool: PgPool) -> Result<Vec<Stock>, ApiError> {
    let country_hashmap_by_name = Country::get_hashmap_by_name(pool).await.unwrap();
    let stock_country_override_hashmap = Stock::get_country_override_hashmap();

    let values = get(false).await?.data;

    let mut stocks: Vec<Stock> = Vec::with_capacity(values.len());
    let mut stock_symbol_set: HashSet<String> = HashSet::new();

    for value in values {
        let symbol = value.symbol;

        if stock_symbol_set.contains(&symbol) {
            continue;
        }

        stock_symbol_set.insert(symbol.clone());

        let country = match stock_country_override_hashmap.get(&symbol) {
            Some(value) => value,
            None => &country_hashmap_by_name[&value.country].code,
        };

        let stock = Stock {
            symbol,
            name: value.name,
            country: country.to_string(),
            stock_type: value.stock_type,
        };
        stocks.push(stock);
    }
    Ok(stocks)
}

pub async fn get_stock_td_plans(pool: PgPool) -> Result<Vec<StockTdPlan>, ApiError> {
    let values = get(true).await?.data;

    let mut stock_td_plans: Vec<StockTdPlan> = Vec::with_capacity(values.len());
    let mut stock_symbol_set: HashSet<String> = HashSet::new();

    for value in values {
        let symbol = value.symbol;
        let access = value.access.unwrap();

        if stock_symbol_set.contains(&symbol) {
            continue;
        }

        stock_symbol_set.insert(symbol.clone());

        let stock_td_plan = StockTdPlan {
            symbol,
            global: access.global,
            plan: access.plan,
        };
        stock_td_plans.push(stock_td_plan);
    }
    Ok(stock_td_plans)
}
