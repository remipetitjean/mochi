use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::{Postgres, QueryBuilder};
use std::collections::HashMap;
use std::fmt;

#[derive(sqlx::Type, Clone, Debug, Deserialize, PartialEq)]
#[sqlx(type_name = "globaltype")]
pub enum GlobalType {
    #[serde(rename = "Basic")]
    #[sqlx(rename = "Basic")]
    Basic,
    #[serde(rename = "Level A")]
    #[sqlx(rename = "Level A")]
    LevelA,
    #[serde(rename = "Level B")]
    #[sqlx(rename = "Level B")]
    LevelB,
    #[serde(rename = "Level C")]
    #[sqlx(rename = "Level C")]
    LevelC,
}

#[derive(sqlx::Type, Clone, Debug, Deserialize, PartialEq)]
#[sqlx(type_name = "plantype")]
pub enum PlanType {
    Basic,
    Grow,
    Pro,
    Enterprise,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StockTdPlan {
    pub symbol: String,
    pub global: GlobalType,
    pub plan: PlanType,
}

impl fmt::Display for StockTdPlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StockTdPlan {}", self.symbol)
    }
}

impl StockTdPlan {
    pub async fn select(pool: PgPool) -> Result<Vec<StockTdPlan>, sqlx::Error> {
        let stock_td_plans = sqlx::query_as!(
            StockTdPlan,
            r#"
            SELECT
                symbol,
                global as "global: GlobalType",
                plan as "plan: PlanType"
            FROM stock_td_plan
            "#
        )
        .fetch_all(&pool)
        .await?;

        Ok(stock_td_plans)
    }

    pub async fn insert_many(
        pool: PgPool,
        stock_td_plans: Vec<StockTdPlan>,
    ) -> Result<(), sqlx::Error> {
        if stock_td_plans.len() == 0 {
            return Ok(());
        }

        let iter = stock_td_plans.chunks(1000);

        for chunk in iter {
            let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
                r#"
                INSERT INTO stock_td_plan (
                    symbol,
                    global,
                    plan
                ) 
                "#,
            );
            query_builder.push_values(chunk, |mut b, stock_td_plan| {
                b.push_bind(&stock_td_plan.symbol)
                    .push_bind(&stock_td_plan.global)
                    .push_bind(&stock_td_plan.plan);
            });

            query_builder.build().execute(&pool).await?;
        }

        Ok(())
    }

    pub async fn update(
        pool: PgPool,
        stock_td_plan: StockTdPlan,
        updated_stock_td_plan: StockTdPlan,
    ) -> Result<(), sqlx::Error> {
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("UPDATE stock_td_plan SET ");

        let mut updated = false;

        if stock_td_plan.global != updated_stock_td_plan.global {
            if updated {
                query_builder.push(", ");
            }
            query_builder
                .push("global = ")
                .push_bind(updated_stock_td_plan.global)
                .push(" ");
            updated = true;
        }

        if stock_td_plan.plan != updated_stock_td_plan.plan {
            if updated {
                query_builder.push(", ");
            }
            query_builder
                .push("plan = ")
                .push_bind(updated_stock_td_plan.plan)
                .push(" ");
            updated = true;
        }

        if updated {
            query_builder.push("where symbol = ");
            query_builder.push_bind(updated_stock_td_plan.symbol);

            query_builder.build().execute(&pool).await?;
        }

        Ok(())
    }

    pub async fn update_many(
        pool: PgPool,
        stock_td_plans: Vec<(StockTdPlan, StockTdPlan)>,
    ) -> Result<(), sqlx::Error> {
        for (stock_td_plan, updated_stock_td_plan) in stock_td_plans {
            StockTdPlan::update(pool.to_owned(), stock_td_plan, updated_stock_td_plan).await?;
        }
        Ok(())
    }
}

impl StockTdPlan {
    pub async fn get_hashmap_by_symbol(
        pool: PgPool,
    ) -> Result<HashMap<String, StockTdPlan>, sqlx::Error> {
        let mut hashmap = HashMap::<String, StockTdPlan>::new();
        let stock_td_plans = StockTdPlan::select(pool).await?;
        for stock_td_plan in stock_td_plans {
            let symbol = &stock_td_plan.symbol;
            hashmap.insert(symbol.to_string(), stock_td_plan);
        }
        Ok(hashmap)
    }
}
