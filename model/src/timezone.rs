use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::{Postgres, QueryBuilder};
use std::fmt;

#[derive(sqlx::Type, Clone, Debug, Deserialize, PartialEq)]
#[sqlx(type_name = "timezonetype")]
pub enum TimezoneType {
    Canonical,
    Type,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Timezone {
    pub code: String,
    pub r#type: TimezoneType,
    pub utc_offset: String,
    pub utc_dst_offset: String,
    pub tz_abbreviation: String,
    pub tz_dst_abbreviation: String,
    pub is_active: bool,
}

impl fmt::Display for Timezone {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Timezone {}", self.code)
    }
}

impl Timezone {
    pub async fn select(pool: PgPool) -> Result<Vec<Timezone>, sqlx::Error> {
        let countries = sqlx::query_as!(
            Timezone,
            r#"
            SELECT
                code,
                type AS "type: _",
                utc_offset,
                utc_dst_offset,
                tz_abbreviation,
                tz_dst_abbreviation,
                is_active
            FROM timezone
            "#
        )
        .fetch_all(&pool)
        .await?;

        Ok(countries)
    }

    pub async fn insert_many(pool: PgPool, countries: Vec<Timezone>) -> Result<(), sqlx::Error> {
        if countries.len() == 0 {
            return Ok(());
        }

        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
            r#"
            INSERT INTO timezone (
                code,
                type AS "type: _",
                utc_offset,
                utc_dst_offset,
                tz_abbreviation,
                tz_dst_abbreviation,
                is_active
            )
         "#,
        );
        query_builder.push_values(countries, |mut b, timezone| {
            b.push_bind(timezone.code)
                .push_bind(timezone.r#type)
                .push_bind(timezone.utc_offset)
                .push_bind(timezone.utc_dst_offset)
                .push_bind(timezone.tz_abbreviation)
                .push_bind(timezone.tz_dst_abbreviation)
                .push_bind(timezone.is_active);
        });

        query_builder.build().execute(&pool).await?;

        Ok(())
    }

    pub async fn update(
        pool: PgPool,
        timezone: Timezone,
        updated_timezone: Timezone,
    ) -> Result<(), sqlx::Error> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE timezone SET ");

        let mut updated = false;

        if timezone.r#type != updated_timezone.r#type {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("type = ");
            query_builder.push_bind(updated_timezone.r#type);
            query_builder.push(" ");
            updated = true;
        }

        if timezone.utc_offset != updated_timezone.utc_offset {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("utc_offset = ");
            query_builder.push_bind(updated_timezone.utc_offset);
            query_builder.push(" ");
            updated = true;
        }

        if timezone.utc_dst_offset != updated_timezone.utc_dst_offset {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("utc_dst_offset = ");
            query_builder.push_bind(updated_timezone.utc_dst_offset);
            query_builder.push(" ");
            updated = true;
        }

        if timezone.tz_abbreviation != updated_timezone.tz_abbreviation {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("tz_abbreviation = ");
            query_builder.push_bind(updated_timezone.tz_abbreviation);
            query_builder.push(" ");
            updated = true;
        }

        if timezone.tz_dst_abbreviation != updated_timezone.tz_dst_abbreviation {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("tz_dst_abbreviation = ");
            query_builder.push_bind(updated_timezone.tz_dst_abbreviation);
            query_builder.push(" ");
            updated = true;
        }

        if timezone.is_active != updated_timezone.is_active {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("is_active = ");
            query_builder.push_bind(updated_timezone.is_active);
            query_builder.push(" ");
            updated = true;
        }

        if updated {
            query_builder.push("where code = ");
            query_builder.push_bind(updated_timezone.code);

            query_builder.build().execute(&pool).await?;
        }

        Ok(())
    }

    pub async fn update_many(
        pool: PgPool,
        countries: Vec<(Timezone, Timezone)>,
    ) -> Result<(), sqlx::Error> {
        for (timezone, updated_timezone) in countries {
            Timezone::update(pool.to_owned(), timezone, updated_timezone).await?;
        }
        Ok(())
    }
}
