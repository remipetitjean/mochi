use crate::prelude::{ActiveStock, Stock};
use crate::stock::{Column, Entity};
use sea_orm::{entity::prelude::*, QuerySelect, Set};
use std::collections::HashSet;

pub struct StockController {}

impl StockController {
    pub async fn insert_many(db: &DbConn, instances: Vec<Stock>) -> Result<(), DbErr> {
        let existing_ids: Vec<String> = Entity::find()
            .select_only()
            .column(Column::Id)
            .into_tuple()
            .all(db)
            .await?;

        let existing_ids_set: HashSet<String> =
            HashSet::from_iter(existing_ids.iter().map(|x| x.to_string()));

        let instances: Vec<Stock> = instances
            .into_iter()
            .filter(|instance| !existing_ids_set.contains(&instance.id))
            .collect::<Vec<Stock>>();

        let active_instances: Vec<ActiveStock> = instances
            .into_iter()
            .map(|instance| ActiveStock {
                id: Set(instance.id.to_owned()),
                name: Set(instance.name.to_owned()),
                currency: Set(instance.currency.to_owned()),
                exchange_id: Set(instance.exchange_id.to_owned()),
                mic_code: Set(instance.mic_code.to_owned()),
                country: Set(instance.country.to_owned()),
                r#type: Set(instance.r#type.to_owned()),
                ..Default::default()
            })
            .rev()
            .collect();

        if active_instances.len() > 0 {
            for v in active_instances.chunks(500) {
                Entity::insert_many(v.to_owned()).exec(db).await?;
            }
            //Entity::insert_many(active_instances).exec(db).await?;
        }

        Ok(())
    }
}
