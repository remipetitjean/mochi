use csv::Reader;
use model::country::Country;
use sqlx::postgres::PgPool;
use std::collections::HashMap;

mod db;

#[tokio::main]
async fn main() {
    let input_countries = read_input_countries();
    let pool = db::get_connection_pool().await.unwrap();

    // retrieve existing countries
    let country_mapping = get_country_mapping(pool.to_owned()).await.unwrap();

    // split into existing and new countries
    let size = input_countries.len();
    let mut existing_countries = Vec::<(Country, Country)>::with_capacity(size);
    let mut new_countries = Vec::<Country>::with_capacity(size);
    for input_country in input_countries {
        match country_mapping.get(&input_country.code) {
            Some(existing_country) => {
                existing_countries.push((existing_country.clone(), input_country))
            }
            None => new_countries.push(input_country),
        };
    }

    // Update existing countries
    Country::insert_many(pool.to_owned(), new_countries)
        .await
        .unwrap();
    Country::update_many(pool, existing_countries)
        .await
        .unwrap();
}

fn read_input_countries() -> Vec<Country> {
    let mut reader = Reader::from_path("seed/country.csv").unwrap();
    let iter = reader.deserialize();
    iter.into_iter().flatten().collect()
}

async fn get_country_mapping(pool: PgPool) -> Result<HashMap<String, Country>, sqlx::Error> {
    let countries = Country::select(pool).await?;
    let mut country_mapping = HashMap::<String, Country>::new();

    for country in countries {
        let code = country.code.clone();
        country_mapping.insert(code, country);
    }

    Ok(country_mapping)
}
