use csv::Reader;
use model::prelude::{
    Country, CountryController, Currency, CurrencyController, DatabaseConnection, Region,
    RegionController,
};

pub async fn update_currency(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path("seed/currency.csv")?;
    let iter = reader.deserialize();
    let currencies: Vec<Currency> = iter.into_iter().flatten().collect();
    CurrencyController::insert_many(&db, currencies).await?;
    Ok(())
}

pub async fn update_region(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path("seed/region.csv")?;
    let iter = reader.deserialize();
    let regions: Vec<Region> = iter.into_iter().flatten().collect();
    RegionController::insert_many(&db, regions).await?;
    Ok(())
}

pub async fn update_country(db: DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = Reader::from_path("seed/country.csv")?;
    let iter = reader.deserialize();
    let countries: Vec<Country> = iter.into_iter().flatten().collect();
    CountryController::insert_many(&db, countries).await?;
    Ok(())
}
