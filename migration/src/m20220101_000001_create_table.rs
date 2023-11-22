use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Stock Type
        manager
            .create_type(
                Type::create()
                    .as_enum(StockType::Table)
                    .values(StockType::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        // Currency
        manager
            .create_table(
                Table::create()
                    .table(Currency::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Currency::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Currency::Name).string().not_null())
                    .col(ColumnDef::new(Currency::Symbol).string())
                    .to_owned(),
            )
            .await?;

        // Region
        manager
            .create_table(
                Table::create()
                    .table(Region::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Region::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Region::Region).string().not_null())
                    .col(ColumnDef::new(Region::SubRegion).string())
                    .col(ColumnDef::new(Region::IntermediateRegion).string())
                    .to_owned(),
            )
            .await?;

        // Country
        manager
            .create_table(
                Table::create()
                    .table(Country::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Country::Id)
                            .char_len(2)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Country::Alpha3).char_len(3).not_null())
                    .col(ColumnDef::new(Country::Name).string().not_null())
                    .col(ColumnDef::new(Country::RegionId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_country_region_id")
                            .from(Country::Table, Country::RegionId)
                            .to(Region::Table, Region::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Exchange
        manager
            .create_table(
                Table::create()
                    .table(Exchange::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Exchange::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Exchange::Name).string().not_null())
                    .col(ColumnDef::new(Exchange::Country).string().not_null())
                    .col(ColumnDef::new(Exchange::Timezone).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Stock
        manager
            .create_table(
                Table::create()
                    .table(Stock::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Stock::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Stock::Name).string().not_null())
                    .col(ColumnDef::new(Stock::Country).string().not_null())
                    .col(
                        ColumnDef::new(Stock::Type)
                            .enumeration(StockType::Table, StockType::iter().skip(1))
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // StockExchange
        manager
            .create_table(
                Table::create()
                    .table(StockExchange::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StockExchange::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(StockExchange::StockId).string().not_null())
                    .col(
                        ColumnDef::new(StockExchange::ExchangeId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(StockExchange::MicCode).string().not_null())
                    .col(
                        ColumnDef::new(StockExchange::CurrencyId)
                            .char_len(2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockExchange::Type)
                            .enumeration(StockType::Table, StockType::iter().skip(1))
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stock_exchange_stock_id")
                            .from(StockExchange::Table, StockExchange::StockId)
                            .to(Stock::Table, Stock::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stock_exchange_exchange_id")
                            .from(StockExchange::Table, StockExchange::ExchangeId)
                            .to(Exchange::Table, Exchange::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stock_exchange_currency_id")
                            .from(StockExchange::Table, StockExchange::CurrencyId)
                            .to(Currency::Table, Currency::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StockExchange::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Stock::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Exchange::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Country::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Region::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Currency::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(StockType::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Currency {
    Table,
    Id,
    Name,
    Symbol,
}

#[derive(DeriveIden)]
enum Region {
    Table,
    Id,
    Region,
    SubRegion,
    IntermediateRegion,
}

#[derive(DeriveIden)]
enum Country {
    Table,
    Id,
    Alpha3,
    Name,
    RegionId,
}

#[derive(DeriveIden)]
enum Exchange {
    Table,
    Id,
    Name,
    Country,
    Timezone,
}

#[derive(Iden, EnumIter)]
pub enum StockType {
    Table,
    #[iden = "Common Stock"]
    CommonStock,
    #[iden = "American Depositary Receipt"]
    AmericanDepositaryReceipt,
    #[iden = "Depositary Receipt"]
    DepositaryReceipt,
    #[iden = "ETF"]
    ETF,
    #[iden = "Global Depositary Receipt"]
    GlobalDepositaryReceipt,
    #[iden = "Limited Partnership"]
    LimitedPartnership,
    #[iden = "Mutual Fund"]
    MutualFund,
    #[iden = "Preferred Stock"]
    PreferredStock,
    #[iden = "REIT"]
    REIT,
    #[iden = "Right"]
    Right,
    #[iden = "Structured Product"]
    StructuredProduct,
    #[iden = "Trust"]
    Trust,
    #[iden = "Unit"]
    Unit,
    #[iden = "Unknown"]
    Unknown,
    #[iden = "Warrant"]
    Warrant,
}

#[derive(DeriveIden)]
enum Stock {
    Table,
    Id,
    Name,
    Country,
    Type,
}

#[derive(DeriveIden)]
enum StockExchange {
    Table,
    Id,
    StockId,
    ExchangeId,
    MicCode,
    CurrencyId,
    Type,
}
