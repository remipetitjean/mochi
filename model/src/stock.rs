use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::{Postgres, QueryBuilder};
use std::fmt;

#[derive(sqlx::Type, Clone, Debug, Deserialize, PartialEq)]
#[sqlx(type_name = "stocktype")]
pub enum StockType {
    #[serde(rename = "American Depositary Receipt")]
    #[sqlx(rename = "American Depositary Receipt")]
    AmericanDepositaryReceipt,
    #[serde(rename = "Closed-end Fund")]
    #[sqlx(rename = "Closed-end Fund")]
    ClosedEndFund,
    #[serde(rename = "Common Stock")]
    #[sqlx(rename = "Common Stock")]
    CommonStock,
    #[serde(rename = "Depositary Receipt")]
    #[sqlx(rename = "Depositary Receipt")]
    DepositaryReceipt,
    #[serde(rename = "ETF")]
    #[sqlx(rename = "ETF")]
    ETF,
    #[serde(rename = "Exchange-Traded Note")]
    #[sqlx(rename = "Exchange-Traded Note")]
    ExchangeTradedNote,
    #[serde(rename = "Global Depositary Receipt")]
    #[sqlx(rename = "Global Depositary Receipt")]
    GlobalDepositaryReceipt,
    #[serde(rename = "Limited Partnership")]
    #[sqlx(rename = "Limited Partnership")]
    LimitedPartnership,
    #[serde(rename = "Mutual Fund")]
    #[sqlx(rename = "Mutual Fund")]
    MutualFund,
    #[serde(rename = "Preferred Stock")]
    #[sqlx(rename = "Preferred Stock")]
    PreferredStock,
    #[serde(rename = "REIT")]
    #[sqlx(rename = "REIT")]
    REIT,
    #[serde(rename = "Right")]
    #[sqlx(rename = "Right")]
    Right,
    #[serde(rename = "Structured Product")]
    #[sqlx(rename = "Structured Product")]
    StructuredProduct,
    #[serde(rename = "Trust")]
    #[sqlx(rename = "Trust")]
    Trust,
    #[serde(rename = "Unit")]
    #[sqlx(rename = "Unit")]
    Unit,
    #[serde(rename = "Warrant")]
    #[sqlx(rename = "Warrant")]
    Warrant,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Stock {
    pub symbol: String,
    pub name: String,
    pub country: String,
    #[serde(rename(deserialize = "type"))]
    pub stock_type: StockType,
}

impl fmt::Display for Stock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stock {}", self.symbol)
    }
}

impl Stock {
    pub async fn select(pool: PgPool) -> Result<Vec<Stock>, sqlx::Error> {
        let stocks = sqlx::query_as!(
            Stock,
            r#"
            SELECT
                symbol,
                name,
                country,
                stock_type as "stock_type: StockType"
            FROM stock
            "#
        )
        .fetch_all(&pool)
        .await?;

        Ok(stocks)
    }

    pub async fn insert_many(pool: PgPool, stocks: Vec<Stock>) -> Result<(), sqlx::Error> {
        if stocks.len() == 0 {
            return Ok(());
        }

        let iter = stocks.chunks(1000);

        for chunk in iter {
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                r#"
                INSERT INTO stock (
                    symbol,
                    name,
                    country,
                    stock_type
                ) 
                "#,
            );
            query_builder.push_values(chunk, |mut b, stock| {
                b.push_bind(&stock.symbol)
                    .push_bind(&stock.name)
                    .push_bind(&stock.country)
                    .push_bind(&stock.stock_type);
            });

            query_builder.build().execute(&pool).await?;
        }

        Ok(())
    }

    pub async fn update(
        pool: PgPool,
        stock: Stock,
        updated_stock: Stock,
    ) -> Result<(), sqlx::Error> {
        let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE stock SET ");

        let mut updated = false;

        if stock.name != updated_stock.name {
            if updated {
                query_builder.push(", ");
            }
            query_builder
                .push("name = ")
                .push_bind(updated_stock.name)
                .push(" ");
            updated = true;
        }

        if stock.country != updated_stock.country {
            if updated {
                query_builder.push(", ");
            }
            query_builder
                .push("country = ")
                .push_bind(updated_stock.country)
                .push(" ");
            updated = true;
        }

        if stock.stock_type != updated_stock.stock_type {
            if updated {
                query_builder.push(", ");
            }
            query_builder
                .push("stock_type = ")
                .push_bind(updated_stock.stock_type)
                .push(" ");
            updated = true;
        }

        if updated {
            query_builder.push("where symbol = ");
            query_builder.push_bind(updated_stock.symbol);

            query_builder.build().execute(&pool).await?;
        }

        Ok(())
    }

    pub async fn update_many(pool: PgPool, stocks: Vec<(Stock, Stock)>) -> Result<(), sqlx::Error> {
        for (stock, updated_stock) in stocks {
            Stock::update(pool.to_owned(), stock, updated_stock).await?;
        }
        Ok(())
    }
}
