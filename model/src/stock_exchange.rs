//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.4

use super::sea_orm_active_enums::StockType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "stock_exchange")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(rename(deserialize = "code"))]
    pub id: i32,
    pub stock_id: String,
    pub exchange_id: String,
    pub mic_code: String,
    pub currency_id: String,
    pub r#type: StockType,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::currency::Entity",
        from = "Column::CurrencyId",
        to = "super::currency::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Currency,
    #[sea_orm(
        belongs_to = "super::exchange::Entity",
        from = "Column::ExchangeId",
        to = "super::exchange::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Exchange,
    #[sea_orm(
        belongs_to = "super::stock::Entity",
        from = "Column::StockId",
        to = "super::stock::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Stock,
}

impl Related<super::currency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Currency.def()
    }
}

impl Related<super::exchange::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Exchange.def()
    }
}

impl Related<super::stock::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stock.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
