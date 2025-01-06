use sea_orm_migration::{
    prelude::*,
    schema::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Lobbies::Table)
                    .add_column_if_not_exists(integer(Lobbies::Size))
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
                    .drop_column(Lobbies::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
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
    Size,
}
