use crate::prelude::{ActiveRegion, Region};
use crate::region::{Column, Entity};
use sea_orm::{entity::prelude::*, QuerySelect, Set};
use std::collections::HashSet;

pub struct RegionController {}

impl RegionController {
    pub async fn insert_many(db: &DbConn, instances: Vec<Region>) -> Result<(), DbErr> {
        let existing_ids: Vec<i32> = Entity::find()
            .select_only()
            .column(Column::Id)
            .into_tuple()
            .all(db)
            .await?;

        let existing_ids_set: HashSet<i32> = HashSet::from_iter(existing_ids);

        // new instances
        let new_instances: Vec<Region> = instances
            .into_iter()
            .filter(|instance| !existing_ids_set.contains(&instance.id))
            .collect::<Vec<Region>>();

        let new_active_instances: Vec<ActiveRegion> = new_instances
            .into_iter()
            .map(|instance| ActiveRegion {
                id: Set(instance.id.to_owned()),
                region: Set(instance.region.to_owned()),
                sub_region: Set(instance.sub_region.to_owned()),
                intermediate_region: Set(instance.intermediate_region.to_owned()),
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
