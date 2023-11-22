//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "stock_type")]
pub enum StockType {
    #[sea_orm(string_value = "American Depositary Receipt")]
    AmericanDepositaryReceipt,
    #[sea_orm(string_value = "Common Stock")]
    CommonStock,
    #[sea_orm(string_value = "Depositary Receipt")]
    DepositaryReceipt,
    #[sea_orm(string_value = "ETF")]
    Etf,
    #[sea_orm(string_value = "Global Depositary Receipt")]
    GlobalDepositaryReceipt,
    #[sea_orm(string_value = "Limited Partnership")]
    LimitedPartnership,
    #[sea_orm(string_value = "Mutual Fund")]
    MutualFund,
    #[sea_orm(string_value = "Preferred Stock")]
    PreferredStock,
    #[sea_orm(string_value = "REIT")]
    Reit,
    #[sea_orm(string_value = "Right")]
    Right,
    #[sea_orm(string_value = "Structured Product")]
    StructuredProduct,
    #[sea_orm(string_value = "Trust")]
    Trust,
    #[sea_orm(string_value = "Unknown")]
    Unknown,
    #[sea_orm(string_value = "Unit")]
    Unit,
    #[sea_orm(string_value = "Warrant")]
    Warrant,
}

impl StockType {
    pub fn from_string(value: &str) -> StockType {
        match value {
            "American Depositary Receipt" => StockType::AmericanDepositaryReceipt,
            "Common Stock" => StockType::CommonStock,
            "Depositary Receipt" => StockType::DepositaryReceipt,
            "ETF" => StockType::Etf,
            "Global Depositary Receipt" => StockType::GlobalDepositaryReceipt,
            "Limited Partnership" => StockType::LimitedPartnership,
            "Mutual Fund" => StockType::MutualFund,
            "Preferred Stock" => StockType::PreferredStock,
            "REIT" => StockType::Reit,
            "Right" => StockType::Right,
            "Structured Product" => StockType::StructuredProduct,
            "Trust" => StockType::Trust,
            "Unit" => StockType::Unit,
            "Warrant" => StockType::Warrant,
            _ => StockType::Unknown,
        }
    }
}
