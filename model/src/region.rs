use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::{Postgres, QueryBuilder};
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Region {
    pub code: String,
    pub region: String,
    pub sub_region: Option<String>,
    pub intermediate_region: Option<String>,
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Region {}", self.code)
    }
}

impl Region {
    pub async fn fetch_all(pool: PgPool) -> Result<Vec<Region>, sqlx::Error> {
        let regions = sqlx::query_as!(
            Region,
            "
            SELECT code, region, sub_region, intermediate_region
            FROM region
            "
        )
        .fetch_all(&pool)
        .await?;

        Ok(regions)
    }

    pub async fn insert_many(pool: PgPool, regions: Vec<Region>) -> Result<(), sqlx::Error> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO region (code, region, sub_region, intermediate_region) ",
        );
        query_builder.push_values(regions, |mut b, region| {
            b.push_bind(region.code)
                .push_bind(region.region)
                .push_bind(region.sub_region)
                .push_bind(region.intermediate_region);
        });

        //let query = query_builder.build();
        let query = query_builder.build();
        query.execute(&pool).await?;

        Ok(())
    }
}
