use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::{Postgres, QueryBuilder};
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
pub struct Exchange {
    pub code: String,
    pub name: String,
    pub country_code: String,
    pub timezone_code: String,
}

impl fmt::Display for Exchange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Exchange {}", self.code)
    }
}

impl Exchange {
    pub async fn select(pool: PgPool) -> Result<Vec<Exchange>, sqlx::Error> {
        let exchanges = sqlx::query_as!(
            Exchange,
            "SELECT code, name, country_code, timezone_code FROM exchange"
        )
        .fetch_all(&pool)
        .await?;

        Ok(exchanges)
    }

    pub async fn insert_many(pool: PgPool, exchanges: Vec<Exchange>) -> Result<(), sqlx::Error> {
        if exchanges.len() == 0 {
            return Ok(());
        }

        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO exchange (code, name, country_code, timezone_code) ");
        query_builder.push_values(exchanges, |mut b, exchange| {
            b.push_bind(exchange.code)
                .push_bind(exchange.name)
                .push_bind(exchange.country_code)
                .push_bind(exchange.timezone_code);
        });

        query_builder.build().execute(&pool).await?;

        Ok(())
    }

    pub async fn update(
        pool: PgPool,
        exchange: Exchange,
        updated_exchange: Exchange,
    ) -> Result<(), sqlx::Error> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE exchange SET ");

        let mut updated = false;

        if exchange.name != updated_exchange.name {
            if updated {
                query_builder.push(", ");
            }
            query_builder
                .push("name = ")
                .push_bind(updated_exchange.name)
                .push(" ");
            updated = true;
        }

        if exchange.country_code != updated_exchange.country_code {
            if updated {
                query_builder.push(", ");
            }
            query_builder
                .push("country_code = ")
                .push_bind(updated_exchange.country_code)
                .push(" ");
            updated = true;
        }

        if exchange.timezone_code != updated_exchange.timezone_code {
            if updated {
                query_builder.push(", ");
            }
            query_builder
                .push("timezone_code = ")
                .push_bind(updated_exchange.timezone_code)
                .push(" ");
            updated = true;
        }

        if updated {
            query_builder.push("where code = ");
            query_builder.push_bind(updated_exchange.code);

            query_builder.build().execute(&pool).await?;
        }

        Ok(())
    }

    pub async fn update_many(
        pool: PgPool,
        exchanges: Vec<(Exchange, Exchange)>,
    ) -> Result<(), sqlx::Error> {
        for (exchange, updated_exchange) in exchanges {
            Exchange::update(pool.to_owned(), exchange, updated_exchange).await?;
        }
        Ok(())
    }
}
