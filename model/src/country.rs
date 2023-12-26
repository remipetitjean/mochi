use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::{Postgres, QueryBuilder};
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
pub struct Country {
    pub code: String,
    pub code_3: String,
    pub name: String,
    pub region_code: String,
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
            SELECT code, code_3, name, region_code
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
            QueryBuilder::new("INSERT INTO country (code, code_3, name, region_code) ");
        query_builder.push_values(countries, |mut b, country| {
            b.push_bind(country.code)
                .push_bind(country.code_3)
                .push_bind(country.name)
                .push_bind(country.region_code);
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

        if country.region_code != updated_country.region_code {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("region_code = ");
            query_builder.push_bind(updated_country.region_code);
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
