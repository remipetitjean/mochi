use crate::country::{Column, Entity};
use crate::prelude::{ActiveCountry, Country};
use sea_orm::{entity::prelude::*, QuerySelect, Set};
use std::collections::HashMap;
use std::collections::HashSet;

pub struct CountryController {}

impl CountryController {
    pub async fn insert_many(db: &DbConn, instances: Vec<Country>) -> Result<(), DbErr> {
        let existing_ids: Vec<String> = Entity::find()
            .select_only()
            .column(Column::Id)
            .into_tuple()
            .all(db)
            .await?;

        let existing_ids_set: HashSet<String> =
            HashSet::from_iter(existing_ids.iter().map(|x| x.to_string()));

        // new instances
        let new_instances: Vec<Country> = instances
            .into_iter()
            .filter(|instance| !existing_ids_set.contains(&instance.id))
            .collect::<Vec<Country>>();

        let new_active_instances: Vec<ActiveCountry> = new_instances
            .into_iter()
            .map(|instance| ActiveCountry {
                id: Set(instance.id.to_owned()),
                alpha3: Set(instance.alpha3.to_owned()),
                name: Set(instance.name.to_owned()),
                region_id: Set(instance.region_id.to_owned()),
                ..Default::default()
            })
            .rev()
            .collect();

        if new_active_instances.len() > 0 {
            Entity::insert_many(new_active_instances).exec(db).await?;
        }

        Ok(())
    }

    pub async fn hashmap(db: &DbConn) -> Result<HashMap<String, Country>, DbErr> {
        let countries = Entity::find().all(db).await?;
        let mut country_map: HashMap<String, Country> = HashMap::new();
        for country in countries {
            country_map.insert(country.id.clone(), country);
        }
        Ok(country_map)
    }
}
