use crate::exchange::{Column, Entity};
use crate::prelude::{ActiveExchange, Exchange};
use sea_orm::{entity::prelude::*, QuerySelect, Set};
use std::collections::HashSet;

pub struct ExchangeController {}

impl ExchangeController {
    pub async fn insert_many(db: &DbConn, instances: Vec<Exchange>) -> Result<(), DbErr> {
        let existing_ids: Vec<String> = Entity::find()
            .select_only()
            .column(Column::Id)
            .into_tuple()
            .all(db)
            .await?;

        let existing_ids_set: HashSet<String> =
            HashSet::from_iter(existing_ids.iter().map(|x| x.to_string()));
        println!("exchanges: {:?}", existing_ids_set);

        let instances: Vec<Exchange> = instances
            .into_iter()
            .filter(|instance| !existing_ids_set.contains(&instance.id))
            .collect::<Vec<Exchange>>();

        let active_instances: Vec<ActiveExchange> = instances
            .into_iter()
            .map(|instance| ActiveExchange {
                id: Set(instance.id.to_owned()),
                name: Set(instance.name.to_owned()),
                country: Set(instance.country.to_owned()),
                timezone: Set(instance.timezone.to_owned()),
                ..Default::default()
            })
            .rev()
            .collect();

        if active_instances.len() > 0 {
            Entity::insert_many(active_instances).exec(db).await?;
        }

        Ok(())
    }
}
