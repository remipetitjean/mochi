use model::prelude::{DatabaseConnection, Exchange, ExchangeController, Stock, StockController};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ExchangeApi {
    data: Vec<Exchange>,
}

#[derive(Debug, Deserialize)]
struct StockApi {
    data: Vec<Stock>,
}

pub async fn update_stock_reference(
    db: DatabaseConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    update_exchange(db.clone()).await?;
    update_stock(db).await?;
    Ok(())
}

async fn update_exchange(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.twelvedata.com/exchanges").await?;
    let exchange_api: ExchangeApi = resp.json().await?;
    let exchanges = exchange_api.data;
    ExchangeController::insert_many(&db, exchanges).await?;
    Ok(())
}

async fn update_stock(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.twelvedata.com/stocks").await?;
    let stock_api: StockApi = resp.json().await?;
    let stocks = stock_api.data;
    StockController::insert_many(&db, stocks).await?;
    Ok(())
}
