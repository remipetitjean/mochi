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
            //.map(|instance| ActiveStockExchange {
            //    stock_id: Set(instance.stock_id.to_owned()),
            //    exchange_id: Set(instance.exchange_id.to_owned()),
            //    mic_code: Set(instance.mic_code.to_owned()),
            //    currency_id: Set(instance.currency_id.to_owned()),
            //    ..Default::default()
            //})
            .rev()
            .collect();

        if new_active_instances.len() > 0 {
            Entity::insert_many(new_active_instances).exec(db).await?;
        }

        Ok(())
    }
}
