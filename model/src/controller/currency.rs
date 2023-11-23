use crate::currency::{Column, Entity};
use crate::prelude::{ActiveCurrency, Currency};
use sea_orm::{entity::prelude::*, QuerySelect, Set};
use std::collections::{HashMap, HashSet};

pub struct CurrencyController {}

impl CurrencyController {
    pub async fn insert_many(db: &DbConn, instances: Vec<Currency>) -> Result<(), DbErr> {
        let existing_ids: Vec<String> = Entity::find()
            .select_only()
            .column(Column::Id)
            .into_tuple()
            .all(db)
            .await?;

        let existing_ids_set: HashSet<String> =
            HashSet::from_iter(existing_ids.iter().map(|x| x.to_string()));

        // new instances
        let new_instances: Vec<Currency> = instances
            .into_iter()
            .filter(|instance| !existing_ids_set.contains(&instance.id))
            .collect::<Vec<Currency>>();

        let new_active_instances: Vec<ActiveCurrency> = new_instances
            .into_iter()
            .map(|instance| ActiveCurrency {
                id: Set(instance.id.to_owned()),
                name: Set(instance.name.to_owned()),
                symbol: Set(instance.symbol.to_owned()),
                ..Default::default()
            })
            .rev()
            .collect();

        if new_active_instances.len() > 0 {
            Entity::insert_many(new_active_instances).exec(db).await?;
        }

        Ok(())
    }

    pub async fn hashmap(db: &DbConn) -> Result<HashMap<String, Currency>, DbErr> {
        let currencies = Entity::find().all(db).await?;
        let mut currency_map: HashMap<String, Currency> = HashMap::new();
        for currency in currencies {
            currency_map.insert(currency.id.clone(), currency);
        }
        Ok(currency_map)
    }

    pub async fn hashmap_name_to_id(db: &DbConn) -> Result<HashMap<String, String>, DbErr> {
        let currencies = Entity::find().all(db).await?;
        let mut currency_map: HashMap<String, String> = HashMap::new();
        for currency in currencies {
            currency_map.insert(currency.name.clone(), currency.id.clone());
        }
        Ok(currency_map)
    }
}
