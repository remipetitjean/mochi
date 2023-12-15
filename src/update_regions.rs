use csv::Reader;
use model::region::Region;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use std::collections::HashMap;

mod db;
mod r#static;

#[tokio::main]
async fn main() {
    let input_regions = read_input_regions();
    let pool = db::get_connection_pool().await.unwrap();

    // retrieve existing regions
    let region_mapping = get_region_mapping(pool.to_owned()).await.unwrap();

    // split into existing and new regions
    let size = input_regions.len();
    let mut existing_regions = Vec::<(Region, Region)>::with_capacity(size);
    let mut new_regions = Vec::<Region>::with_capacity(size);
    for input_region in input_regions {
        match region_mapping.get(&input_region.code) {
            Some(existing_region) => existing_regions.push((existing_region, input_region)),
            None => new_regions.push(input_region),
        };
    }

    Region::insert_many(pool, new_regions).await.unwrap();
}

fn read_input_regions() -> Vec<Region> {
    let mut reader = Reader::from_path("seed/region.csv").unwrap();
    let iter = reader.deserialize();
    iter.into_iter().flatten().collect()
}

async fn get_region_mapping(pool: PgPool) -> Result<HashMap<String, Region>, sqlx::Error> {
    let regions = Region::fetch_all(pool).await?;
    let mut region_mapping = HashMap::<String, Region>::new();

    for region in regions {
        let code = region.code.clone();
        region_mapping.insert(code, region);
    }

    Ok(region_mapping)
}

fn _model_mapping<T>(model_vec: Vec<T>, _key: String) {
    let _mapping = HashMap::<String, T>::new();
    for _model in model_vec {
        // cannot get an object attribute at compile time
        // need to write a macro
        todo!()
    }
}
