use sea_orm::{
    prelude::*,
    Linked,
};
pub struct LobbyToUser;

impl Linked for LobbyToUser {
    type FromEntity = super::lobbies::Entity;
    type ToEntity = super::users::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            super::users_lobbies::Relation::Users.def().rev(),
            super::users_lobbies::Relation::Lobbies.def(),
        ]
    }
}

pub struct UserToLobby;

impl Linked for UserToLobby {
    type FromEntity = super::users::Entity;
    type ToEntity = super::lobbies::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            super::users_lobbies::Relation::Lobbies.def().rev(),
            super::users_lobbies::Relation::Users.def(),
        ]
    }
}

pub struct LobbyToBan;

impl Linked for LobbyToBan {
    type FromEntity = super::lobbies::Entity;
    type ToEntity = super::lobby_bans::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            super::lobby_bans::Relation::Lobbies.def().rev(),
            super::lobby_bans::Relation::Users.def(),
        ]
    }
}

pub struct BanToLobby;

impl Linked for BanToLobby {
    type FromEntity = super::lobby_bans::Entity;
    type ToEntity = super::lobbies::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![
            super::lobby_bans::Relation::Users.def().rev(),
            super::lobby_bans::Relation::Lobbies.def(),
        ]
    }
}
