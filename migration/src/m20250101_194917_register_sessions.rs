use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(RegisterSessions::Table)
                    .col(pk_auto(RegisterSessions::Id))
                    .col(uuid_uniq(RegisterSessions::SessionId))
                    .col(timestamp(RegisterSessions::Expiry))
                    .col(string(RegisterSessions::VerificationCode))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RegisterSessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RegisterSessions {
    Table,
    Id,
    SessionId,
    Expiry,
    VerificationCode,
    
}

