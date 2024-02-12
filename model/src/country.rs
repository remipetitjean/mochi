use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::{Postgres, QueryBuilder};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
pub struct Country {
    pub code: String,
    pub code_3: String,
    pub name: String,
    pub region: String,
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Country {}", self.code)
    }
}

impl Country {
    pub async fn select(pool: PgPool) -> Result<Vec<Country>, sqlx::Error> {
        let countries = sqlx::query_as!(
            Country,
            "
            SELECT code, code_3, name, region
            FROM country
            "
        )
        .fetch_all(&pool)
        .await?;

        Ok(countries)
    }

    pub async fn insert_many(pool: PgPool, countries: Vec<Country>) -> Result<(), sqlx::Error> {
        if countries.len() == 0 {
            return Ok(());
        }

        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO country (code, code_3, name, region) ");
        query_builder.push_values(countries, |mut b, country| {
            b.push_bind(country.code)
                .push_bind(country.code_3)
                .push_bind(country.name)
                .push_bind(country.region);
        });

        query_builder.build().execute(&pool).await?;

        Ok(())
    }

    pub async fn update(
        pool: PgPool,
        country: Country,
        updated_country: Country,
    ) -> Result<(), sqlx::Error> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE country SET ");

        let mut updated = false;

        if country.code_3 != updated_country.code_3 {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("code_3 = ");
            query_builder.push_bind(updated_country.code_3);
            query_builder.push(" ");
            updated = true;
        }

        if country.name != updated_country.name {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("name = ");
            query_builder.push_bind(updated_country.name);
            query_builder.push(" ");
            updated = true;
        }

        if country.region != updated_country.region {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("region = ");
            query_builder.push_bind(updated_country.region);
            query_builder.push(" ");
            updated = true;
        }

        if updated {
            query_builder.push("where code = ");
            query_builder.push_bind(updated_country.code);

            query_builder.build().execute(&pool).await?;
        }

        Ok(())
    }

    pub async fn update_many(
        pool: PgPool,
        countries: Vec<(Country, Country)>,
    ) -> Result<(), sqlx::Error> {
        for (country, updated_country) in countries {
            Country::update(pool.to_owned(), country, updated_country).await?;
        }
        Ok(())
    }
}

impl Country {
    pub async fn get_hashmap_by_name(
        pool: PgPool,
    ) -> Result<HashMap<String, Country>, sqlx::Error> {
        let countries = Country::select(pool).await?;

        let mut hashmap: HashMap<String, Country> = HashMap::new();
        for country in countries {
            hashmap.insert(country.name.clone(), country);
        }

        // manual insertion: Bosnia and Herzegovina
        let country = hashmap.get("Bosnia and Herzegovina");
        match country {
            Some(country) => {
                hashmap.insert("Bosnia & Herzegovina".to_string(), country.clone());
            }
            None => {}
        }

        // manual insertion: Czechia
        let country = hashmap.get("Czechia");
        match country {
            Some(country) => {
                hashmap.insert("Czech Republic".to_string(), country.clone());
            }
            None => {}
        }

        // manual insertion: Palestine, State of
        let country = hashmap.get("Palestine, State of");
        match country {
            Some(country) => {
                hashmap.insert("Palestinian Territories".to_string(), country.clone());
            }
            None => {}
        }

        // manual insertion: Russian Federation
        let country = hashmap.get("Russian Federation");
        match country {
            Some(country) => {
                hashmap.insert("Russia".to_string(), country.clone());
            }
            None => {}
        }

        // manual insertion: Korea, Republic of
        let country = hashmap.get("Korea, Republic of");
        match country {
            Some(country) => {
                hashmap.insert("South Korea".to_string(), country.clone());
            }
            None => {}
        }

        // manual insertion: Taiwan, Republic of China
        let country = hashmap.get("Taiwan, Republic of China");
        match country {
            Some(country) => {
                hashmap.insert("Taiwan".to_string(), country.clone());
            }
            None => {}
        }

        // manual insertion: Tanzania, United Republic of
        let country = hashmap.get("Tanzania, United Republic of");
        match country {
            Some(country) => {
                hashmap.insert("Tanzania".to_string(), country.clone());
            }
            None => {}
        }

        // manual insertion: United Kingdom of Great Britain and Northern Ireland
        let country = hashmap.get("United Kingdom of Great Britain and Northern Ireland");
        match country {
            Some(country) => {
                hashmap.insert("United Kingdom".to_string(), country.clone());
            }
            None => {}
        }

        // manual insertion: United States of America
        let country = hashmap.get("United States of America");
        match country {
            Some(country) => {
                hashmap.insert("United States".to_string(), country.clone());
            }
            None => {}
        }

        // manual insertion: Venezuela (Bolivarian Republic of)
        let country = hashmap.get("Venezuela (Bolivarian Republic of)");
        match country {
            Some(country) => {
                hashmap.insert("Venezuela".to_string(), country.clone());
            }
            None => {}
        }

        // manual insertion: Viet Nam
        let country = hashmap.get("Viet Nam");
        match country {
            Some(country) => {
                hashmap.insert("Vietnam".to_string(), country.clone());
            }
            None => {}
        }

        Ok(hashmap)
    }

    pub async fn get_hashmap_by_code(
        pool: PgPool,
    ) -> Result<HashMap<String, Country>, sqlx::Error> {
        let countries = Country::select(pool).await?;

        let mut hashmap: HashMap<String, Country> = HashMap::new();
        for country in countries {
            hashmap.insert(country.code.clone(), country);
        }

        Ok(hashmap)
    }
}
