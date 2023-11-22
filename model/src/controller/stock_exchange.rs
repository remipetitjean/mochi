use crate::prelude::{ActiveStockExchange, StockExchange};
use crate::stock_exchange::Entity;
use sea_orm::{entity::prelude::*, Set};

pub struct StockExchangeController {}

impl StockExchangeController {
    pub async fn insert_many(db: &DbConn, instances: Vec<StockExchange>) -> Result<(), DbErr> {
        let new_active_instances: Vec<ActiveStockExchange> = instances
            .into_iter()
            .map(|instance| ActiveStockExchange {
                id: Set(instance.id.to_owned()),
                stock_id: Set(instance.stock_id.to_owned()),
                exchange_id: Set(instance.exchange_id.to_owned()),
                mic_code: Set(instance.mic_code.to_owned()),
                currency_id: Set(instance.currency_id.to_owned()),
                r#type: Set(instance.r#type.to_owned()),
                ..Default::default()
            })
            .rev()
            .collect();

        if new_active_instances.len() > 0 {
            Entity::insert_many(new_active_instances).exec(db).await?;
        }

        Ok(())
    }
}
