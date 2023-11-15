//https://api.twelvedata.com/exchanges?format=csv
//https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
mod r#static;
mod stock;

use crate::r#static::update_currency;
use crate::stock::update_stock_reference;
use model::prelude::get_database_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db = get_database_connection().await;
    //update_currency(db).await?;
    update_stock_reference(db).await?;

    Ok(())
}
