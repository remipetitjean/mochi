use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::{Postgres, QueryBuilder};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, Deserialize)]
pub struct StockPrice {
    pub symbol: String,
    pub eod: NaiveDate,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StockEod {
    pub symbol: String,
    pub eod: Option<NaiveDate>,
}

impl fmt::Display for StockPrice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StockPrice {}", self.symbol)
    }
}

impl StockPrice {
    pub async fn select(pool: PgPool) -> Result<Vec<StockPrice>, sqlx::Error> {
        let stock_prices = sqlx::query_as!(
            StockPrice,
            r#"
            SELECT
                symbol,
                eod,
                open,
                high,
                low,
                close,
                volume
            FROM stock_price
            "#
        )
        .fetch_all(&pool)
        .await?;

        Ok(stock_prices)
    }

    pub async fn insert_many(
        pool: PgPool,
        stock_prices: Vec<StockPrice>,
    ) -> Result<(), sqlx::Error> {
        if stock_prices.len() == 0 {
            return Ok(());
        }

        let iter = stock_prices.chunks(1000);

        for chunk in iter {
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                r#"
                INSERT INTO stock_price (
                    symbol,
                    eod,
                    open,
                    high,
                    low,
                    close,
                    volume
                ) 
                "#,
            );
            query_builder.push_values(chunk, |mut b, stock_price| {
                b.push_bind(&stock_price.symbol)
                    .push_bind(&stock_price.eod)
                    .push_bind(&stock_price.open)
                    .push_bind(&stock_price.high)
                    .push_bind(&stock_price.low)
                    .push_bind(&stock_price.close)
                    .push_bind(&stock_price.volume);
            });

            query_builder.build().execute(&pool).await?;
        }

        Ok(())
    }

    pub async fn get_eod_hashmap(
        pool: PgPool,
    ) -> Result<HashMap<String, NaiveDate>, sqlx::error::Error> {
        let stock_prices: Vec<StockEod> = sqlx::query_as!(
            StockEod,
            r#"
            SELECT
                symbol,
                max(eod) as eod
            FROM stock_price
            GROUP BY symbol
            "#
        )
        .fetch_all(&pool)
        .await?;

        let mut hashmap: HashMap<String, NaiveDate> = HashMap::new();
        for stock_price in stock_prices {
            hashmap.insert(
                stock_price.symbol,
                stock_price.eod.expect("should not be none"),
            );
        }
        Ok(hashmap)
    }
}
