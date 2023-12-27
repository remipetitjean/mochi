use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::{Postgres, QueryBuilder};
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: Option<String>,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Currency {}", self.code)
    }
}

impl Currency {
    pub async fn select(pool: PgPool) -> Result<Vec<Currency>, sqlx::Error> {
        let currencies = sqlx::query_as!(Currency, "SELECT code, name, symbol FROM currency")
            .fetch_all(&pool)
            .await?;

        Ok(currencies)
    }

    pub async fn insert_many(pool: PgPool, currencies: Vec<Currency>) -> Result<(), sqlx::Error> {
        if currencies.len() == 0 {
            return Ok(());
        }

        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO currency (code, name, symbol) ");
        query_builder.push_values(currencies, |mut b, currency| {
            b.push_bind(currency.code)
                .push_bind(currency.name)
                .push_bind(currency.symbol);
        });

        query_builder.build().execute(&pool).await?;

        Ok(())
    }

    pub async fn update(
        pool: PgPool,
        currency: Currency,
        updated_currency: Currency,
    ) -> Result<(), sqlx::Error> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE currency SET ");

        let mut updated = false;

        if currency.name != updated_currency.name {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("name = ");
            query_builder.push_bind(updated_currency.name);
            query_builder.push(" ");
            updated = true;
        }

        if currency.symbol != updated_currency.symbol {
            if updated {
                query_builder.push(", ");
            }
            query_builder.push("symbol = ");
            query_builder.push_bind(updated_currency.symbol);
            query_builder.push(" ");
            updated = true;
        }

        if updated {
            query_builder.push("where code = ");
            query_builder.push_bind(updated_currency.code);

            query_builder.build().execute(&pool).await?;
        }

        Ok(())
    }

    pub async fn update_many(
        pool: PgPool,
        currencies: Vec<(Currency, Currency)>,
    ) -> Result<(), sqlx::Error> {
        for (currency, updated_currency) in currencies {
            Currency::update(pool.to_owned(), currency, updated_currency).await?;
        }
        Ok(())
    }
}
