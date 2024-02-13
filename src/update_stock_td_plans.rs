mod db;
mod twelvedata;

use self::twelvedata::endpoint::stocks::get_stock_td_plans;
use model::stock_td_plan::StockTdPlan;

#[tokio::main]
async fn main() {
    let pool = db::get_connection_pool().await.unwrap();

    let stock_td_plans = get_stock_td_plans(pool.to_owned()).await.unwrap();
    let existing_stock_td_plan_mapping = StockTdPlan::get_hashmap_by_symbol(pool.to_owned())
        .await
        .unwrap();

    // split into existing and new stocks
    let size = stock_td_plans.len();
    let mut existing_stock_td_plans = Vec::<(StockTdPlan, StockTdPlan)>::with_capacity(size);
    let mut new_stock_td_plans = Vec::<StockTdPlan>::with_capacity(size);
    for stock_td_plan in stock_td_plans {
        match existing_stock_td_plan_mapping.get(&stock_td_plan.symbol) {
            Some(existing_stock_td_plan) => {
                existing_stock_td_plans.push((existing_stock_td_plan.clone(), stock_td_plan))
            }
            None => new_stock_td_plans.push(stock_td_plan),
        };
    }

    // Update existing stock_td_plans
    StockTdPlan::insert_many(pool.to_owned(), new_stock_td_plans)
        .await
        .unwrap();
    StockTdPlan::update_many(pool, existing_stock_td_plans)
        .await
        .unwrap();
}
