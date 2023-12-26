use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::{Postgres, QueryBuilder};
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
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
    pub async fn select(pool: PgPool) -> Result<Vec<Region>, sqlx::Error> {
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
        if regions.len() == 0 {
            return Ok(());
        }

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            "INSERT INTO region (code, region, sub_region, intermediate_region) ",
        );
        query_builder.push_values(regions, |mut b, region| {
            b.push_bind(region.code)
                .push_bind(region.region)
                .push_bind(region.sub_region)
                .push_bind(region.intermediate_region);
        });

        query_builder.build().execute(&pool).await?;

        Ok(())
    }

    pub async fn update(
        pool: PgPool,
        region: Region,
        updated_region: Region,
    ) -> Result<(), sqlx::Error> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE region SET ");

        let mut updated = false;

        if region.sub_region != updated_region.sub_region {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("region = ");
            query_builder.push_bind(updated_region.region);
            query_builder.push(" ");
            updated = true;
        }

        if region.sub_region != updated_region.sub_region {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("sub_region = ");
            query_builder.push_bind(updated_region.sub_region);
            query_builder.push(" ");
            updated = true;
        }

        if region.intermediate_region != updated_region.intermediate_region {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("intermediate_region = ");
            query_builder.push_bind(updated_region.intermediate_region);
            query_builder.push(" ");
            updated = true;
        }

        if updated {
            query_builder.push("where code = ");
            query_builder.push_bind(updated_region.code);

            query_builder.build().execute(&pool).await?;
        }

        Ok(())
    }

    pub async fn update_many(
        pool: PgPool,
        regions: Vec<(Region, Region)>,
    ) -> Result<(), sqlx::Error> {
        for (region, updated_region) in regions {
            Region::update(pool.to_owned(), region, updated_region).await?;
        }
        Ok(())
    }
}
