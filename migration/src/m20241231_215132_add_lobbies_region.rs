use {
    extension::postgres::Type,
    sea_orm_migration::{prelude::*, schema::*},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        const REGION_VALUES: [Regions; 6] = [
            Regions::AS,
            Regions::EER,
            Regions::EU,
            Regions::NA,
            Regions::OC,
            Regions::SA,
        ];

        manager
            .create_type(
                Type::create()
                    .as_enum(Regions::Type)
                    .values(REGION_VALUES)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lobbies::Table)
                    .add_column_if_not_exists(enumeration(
                        Lobbies::Region,
                        Regions::Type,
                        REGION_VALUES,
                    ))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Lobbies::Table)
                    .drop_column(Lobbies::Region)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
#[allow(clippy::upper_case_acronyms)]
pub enum Regions {
    #[sea_orm(iden = "regions")]
    Type,
    #[sea_orm(iden = "EER")]
    EER, // Eastern Europe (including Russia)
    #[sea_orm(iden = "AS")]
    AS, // Asia
    #[sea_orm(iden = "OC")]
    OC, // Oceania
    #[sea_orm(iden = "NA")]
    NA, // North America
    #[sea_orm(iden = "SA")]
    SA, // South America
    #[sea_orm(iden = "EU")]
    EU, // Europe
}

#[derive(DeriveIden)]
#[allow(unused)]
enum Lobbies {
    Table,
    Id,
    Expiry,
    Region,
}
