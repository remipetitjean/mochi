use csv::Reader;
use model::prelude::{Currency, CurrencyController, DatabaseConnection};

pub async fn update_currency(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path("seed/currency.csv")?;
    let iter = reader.deserialize();
    let currencies: Vec<Currency> = iter.into_iter().flatten().collect();
    CurrencyController::insert_many(&db, currencies).await?;
    Ok(())
}
