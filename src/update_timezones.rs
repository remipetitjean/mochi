use csv::Reader;
use model::timezone::Timezone;
use sqlx::postgres::PgPool;
use std::collections::HashMap;

mod db;

#[tokio::main]
async fn main() {
    let input_countries = read_input_countries();

    let pool = db::get_connection_pool().await.unwrap();

    // retrieve existing countries
    let timezone_mapping = get_timezone_mapping(pool.to_owned()).await.unwrap();

    // split into existing and new countries
    let size = input_countries.len();
    let mut existing_countries = Vec::<(Timezone, Timezone)>::with_capacity(size);
    let mut new_countries = Vec::<Timezone>::with_capacity(size);
    for input_timezone in input_countries {
        match timezone_mapping.get(&input_timezone.code) {
            Some(existing_timezone) => {
                existing_countries.push((existing_timezone.clone(), input_timezone))
            }
            None => new_countries.push(input_timezone),
        };
    }

    // Update existing countries
    Timezone::insert_many(pool.to_owned(), new_countries)
        .await
        .unwrap();
    Timezone::update_many(pool, existing_countries)
        .await
        .unwrap();
}

fn read_input_countries() -> Vec<Timezone> {
    let mut reader = Reader::from_path("seed/timezone.csv").unwrap();
    let iter = reader.deserialize();
    iter.into_iter().flatten().collect()
}

async fn get_timezone_mapping(pool: PgPool) -> Result<HashMap<String, Timezone>, sqlx::Error> {
    let countries = Timezone::select(pool).await?;
    let mut timezone_mapping = HashMap::<String, Timezone>::new();

    for timezone in countries {
        let code = timezone.code.clone();
        timezone_mapping.insert(code, timezone);
    }

    Ok(timezone_mapping)
}
