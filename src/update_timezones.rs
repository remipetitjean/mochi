use csv::Reader;
use model::timezone::Timezone;
use sqlx::postgres::PgPool;
use std::collections::HashMap;

mod db;

#[tokio::main]
async fn main() {
    let input_timezones = read_input_timezones();

    let pool = db::get_connection_pool().await.unwrap();

    // retrieve existing timezones
    let timezone_mapping = get_timezone_mapping(pool.to_owned()).await.unwrap();

    // split into existing and new timezones
    let size = input_timezones.len();
    let mut existing_timezones = Vec::<(Timezone, Timezone)>::with_capacity(size);
    let mut new_timezones = Vec::<Timezone>::with_capacity(size);
    for input_timezone in input_timezones {
        match timezone_mapping.get(&input_timezone.code) {
            Some(existing_timezone) => {
                existing_timezones.push((existing_timezone.clone(), input_timezone))
            }
            None => new_timezones.push(input_timezone),
        };
    }

    // Update existing timezones
    Timezone::insert_many(pool.to_owned(), new_timezones)
        .await
        .unwrap();
    Timezone::update_many(pool, existing_timezones)
        .await
        .unwrap();
}

fn read_input_timezones() -> Vec<Timezone> {
    let mut reader = Reader::from_path("seed/timezone.csv").unwrap();
    let iter = reader.deserialize();
    iter.into_iter().flatten().collect()
}

async fn get_timezone_mapping(pool: PgPool) -> Result<HashMap<String, Timezone>, sqlx::Error> {
    let timezones = Timezone::select(pool).await?;
    let mut timezone_mapping = HashMap::<String, Timezone>::new();

    for timezone in timezones {
        let code = timezone.code.clone();
        timezone_mapping.insert(code, timezone);
    }

    Ok(timezone_mapping)
}
