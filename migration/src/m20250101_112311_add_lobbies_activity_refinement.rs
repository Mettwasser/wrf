use {
    extension::postgres::Type,
    sea_orm_migration::{prelude::*, schema::*},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        const REFINEMENT_VALUES: [RelicRefinement; 3] = [
            RelicRefinement::Intact,
            RelicRefinement::Flawless,
            RelicRefinement::Radiant,
        ];

        manager
            .create_type(
                Type::create()
                    .as_enum(RelicRefinement::Type)
                    .values(REFINEMENT_VALUES)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Lobbies::Table)
                    .add_column_if_not_exists(enumeration(
                        Lobbies::Refinement,
                        RelicRefinement::Type,
                        REFINEMENT_VALUES,
                    ))
                    .add_column_if_not_exists(string(Lobbies::Activity))
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
                    .drop_column(Lobbies::Refinement)
                    .drop_column(Lobbies::Activity)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(RelicRefinement::Type)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum RelicRefinement {
    #[sea_orm(iden = "relic_refinement")]
    Type,
    #[sea_orm(iden = "Intact")]
    Intact,
    #[sea_orm(iden = "Flawless")]
    Flawless,
    #[sea_orm(iden = "Radiant")]
    Radiant,
}

#[derive(DeriveIden)]
#[allow(unused)]
enum Lobbies {
    Table,
    Id,
    Expiry,
    Region,
    Refinement,
    Activity,
}
