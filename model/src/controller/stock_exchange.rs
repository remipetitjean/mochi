use crate::prelude::{ActiveStockExchange, NewStockExchange, StockExchange};
use crate::stock_exchange::Entity;
use sea_orm::entity::prelude::*;
use sea_orm::IntoActiveModel;
use std::collections::HashSet;

pub struct StockExchangeController {}

impl StockExchangeController {
    pub async fn insert_many(db: &DbConn, instances: Vec<NewStockExchange>) -> Result<(), DbErr> {
        // Filter out existing instances
        let existing_instances: Vec<StockExchange> = Entity::find().all(db).await?;
        let existing_ids_set: HashSet<(String, String)> = HashSet::from_iter(
            existing_instances
                .iter()
                .map(|x| (x.stock_id.to_string(), x.exchange_id.to_string())),
        );
        let instances: Vec<NewStockExchange> = instances
            .into_iter()
            .filter(|instance| {
                !existing_ids_set.contains(&(
                    instance.stock_id.to_owned(),
                    instance.exchange_id.to_owned(),
                ))
            })
            .collect();

        let new_active_instances: Vec<ActiveStockExchange> = instances
            .into_iter()
            .map(|instance| instance.into_active_model())
            .rev()
            .collect();

        //if new_active_instances.len() > 0 {

        let new_active_instances_chunked: Vec<Vec<ActiveStockExchange>> = new_active_instances
            .chunks(10_000)
            .map(|x| x.to_vec())
            .collect();

        for chunk in new_active_instances_chunked {
            println!("About to insert {} stock exchanges", chunk.len());
            Entity::insert_many(chunk).exec(db).await?;
        }

        //Entity::insert_many(new_active_instances).exec(db).await?;
        //}

        Ok(())
    }
}
