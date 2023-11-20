//https://api.twelvedata.com/exchanges?format=csv
//https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html
mod r#static;
mod stock;

use crate::r#static::{update_country, update_currency, update_region};
use crate::stock::update_stock_reference;
use model::prelude::get_database_connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let db = get_database_connection().await;

    // static
    update_currency(db.clone()).await?;
    update_region(db.clone()).await?;
    update_country(db).await?;

    //update_stock_reference(db).await?;

    Ok(())
}
