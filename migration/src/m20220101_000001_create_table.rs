use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Exchange::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Exchange {
    Table,
    Id,
    Name,
    Country,
    Timezone,
}
