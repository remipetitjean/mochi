use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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

        manager
            .create_table(
                Table::create()
                    .table(Stock::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Stock::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Stock::Name).string().not_null())
                    .col(ColumnDef::new(Stock::Currency).string().not_null())
                    .col(ColumnDef::new(Stock::ExchangeId).string().not_null())
                    .col(ColumnDef::new(Stock::MicCode).string().not_null())
                    .col(ColumnDef::new(Stock::Country).string().not_null())
                    .col(ColumnDef::new(Stock::Type).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_stock_exchange_id")
                            .from(Stock::Table, Stock::ExchangeId)
                            .to(Exchange::Table, Exchange::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Stock::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Exchange::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Currency::Table).to_owned())
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
enum Exchange {
    Table,
    Id,
    Name,
    Country,
    Timezone,
}

#[derive(DeriveIden)]
enum Stock {
    Table,
    Id,
    Name,
    Currency,
    ExchangeId,
    MicCode,
    Country,
    Type,
}
