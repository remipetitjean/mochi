use csv::Reader;
use model::currency::Currency;
use sqlx::postgres::PgPool;
use std::collections::HashMap;

mod db;

#[tokio::main]
async fn main() {
    let input_currencies = read_input_currencies();
    let pool = db::get_connection_pool().await.unwrap();

    // retrieve existing currencies
    let currency_mapping = get_currency_mapping(pool.to_owned()).await.unwrap();

    // split into existing and new currencies
    let size = input_currencies.len();
    let mut existing_currencies = Vec::<(Currency, Currency)>::with_capacity(size);
    let mut new_currencies = Vec::<Currency>::with_capacity(size);
    for input_currency in input_currencies {
        match currency_mapping.get(&input_currency.code) {
            Some(existing_currency) => {
                existing_currencies.push((existing_currency.clone(), input_currency))
            }
            None => new_currencies.push(input_currency),
        };
    }

    // Update existing currencies
    Currency::insert_many(pool.to_owned(), new_currencies)
        .await
        .unwrap();
    Currency::update_many(pool, existing_currencies)
        .await
        .unwrap();
}

fn read_input_currencies() -> Vec<Currency> {
    let mut reader = Reader::from_path("seed/currency.csv").unwrap();
    let iter = reader.deserialize();
    iter.into_iter().flatten().collect()
}

async fn get_currency_mapping(pool: PgPool) -> Result<HashMap<String, Currency>, sqlx::Error> {
    let currencies = Currency::select(pool).await?;
    let mut currency_mapping = HashMap::<String, Currency>::new();

    for currency in currencies {
        let code = currency.code.clone();
        currency_mapping.insert(code, currency);
    }

    Ok(currency_mapping)
}
