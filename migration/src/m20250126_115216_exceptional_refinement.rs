#![allow(unused)]
use sea_orm_migration::prelude::{
    extension::postgres::Type,
    *,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.alter_type(
            Type::alter()
                .name(RelicRefinement::Type)
                .add_value(RelicRefinement::Exceptional),
        )
        .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum RelicRefinement {
    #[sea_orm(iden = "relic_refinement")]
    Type,
    #[sea_orm(iden = "Intact")]
    Intact,
    #[sea_orm(iden = "Exceptional")]
    Exceptional,
    #[sea_orm(iden = "Flawless")]
    Flawless,
    #[sea_orm(iden = "Radiant")]
    Radiant,
}
