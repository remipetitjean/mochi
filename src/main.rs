//https://api.twelvedata.com/exchanges?format=csv
//https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
pub mod stock;

use crate::stock::update_stock_reference;
use model::prelude::get_database_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db = get_database_connection().await;
    update_stock_reference(db).await?;
    
    Ok(())
}
