use crate::prelude::{ActiveStockExchange, NewStockExchange};
use crate::stock_exchange::Entity;
use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;

pub struct StockExchangeController {}

impl StockExchangeController {
    pub async fn insert_many(db: &DbConn, instances: Vec<NewStockExchange>) -> Result<(), DbErr> {
        let new_active_instances: Vec<ActiveStockExchange> = instances
            .into_iter()
            .map(|instance| instance.into_active_model())
            .rev()
            .collect();

        if new_active_instances.len() > 0 {
            Entity::insert_many(new_active_instances).exec(db).await?;
        }

        Ok(())
    }
}
