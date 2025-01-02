use loco_rs::schema::table_auto_tz;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto_tz(UsersLobbies::Table)
                    .primary_key(
                        Index::create()
                            .name("idx-users_lobbies-refs-pk")
                            .table(UsersLobbies::Table)
                            .col(UsersLobbies::UserId)
                            .col(UsersLobbies::LobbyId)
                            ,
                    )
                    .col(integer(UsersLobbies::UserId))
                    .col(integer(UsersLobbies::LobbyId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-users_lobbies-user_ids")
                            .from(UsersLobbies::Table, UsersLobbies::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-users_lobbies-lobby_ids")
                            .from(UsersLobbies::Table, UsersLobbies::LobbyId)
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
            .drop_table(Table::drop().table(UsersLobbies::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UsersLobbies {
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
