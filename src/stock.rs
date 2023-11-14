use model::prelude::{DatabaseConnection, Exchange, ExchangeController};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ExchangeApi {
    data: Vec<Exchange>,
}

pub async fn update_stock_reference(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    update_exchange(db).await?;
    Ok(())
}

async fn update_exchange(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.twelvedata.com/exchanges").await?;
    let exchange_api: ExchangeApi = resp.json().await?;
    let exchanges = exchange_api.data;
    ExchangeController::insert_many(&db, exchanges).await?;
    Ok(())
}
