use loco_rs::schema::table_auto_tz;
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
            .create_table(
                table_auto_tz(LobbyBans::Table)
                    .primary_key(
                        Index::create()
                            .name("idx-lobby_bans-refs-pk")
                            .table(LobbyBans::Table)
                            .col(LobbyBans::UserId)
                            .col(LobbyBans::LobbyId),
                    )
                    .col(integer(LobbyBans::UserId))
                    .col(integer(LobbyBans::LobbyId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-lobby_bans-user_ids")
                            .from(LobbyBans::Table, LobbyBans::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-lobby_bans-lobby_ids")
                            .from(LobbyBans::Table, LobbyBans::LobbyId)
                            .to(Lobbies::Table, Lobbies::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LobbyBans::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum LobbyBans {
    Table,
    UserId,
    LobbyId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
#[derive(DeriveIden)]
enum Lobbies {
    Table,
    Id,
}
