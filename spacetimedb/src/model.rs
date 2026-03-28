#![allow(clippy::pedantic, deprecated)]

use spacetimedb::{
    Identity,
    Timestamp,
};

use crate::{
    permissions::Permissions,
    types::{
        Region,
        RelicRefinement,
        RotationType,
    },
    user_flags::UserFlags,
};

#[spacetimedb::table(accessor = user_id)]
pub struct UserId {
    #[primary_key]
    pub identity: Identity,

    #[unique]
    #[auto_inc]
    pub id: u32,
}

#[spacetimedb::table(accessor = user, public)]
pub struct User {
    #[primary_key]
    pub id: u32,

    #[unique]
    pub name: String,
}

#[spacetimedb::table(accessor = user_details, public)]
pub struct UserDetails {
    #[primary_key]
    pub user_id: u32,

    pub flags: UserFlags,

    pub permissions: Permissions,
}

#[spacetimedb::table(accessor = lobby, public)]
pub struct Lobby {
    #[primary_key]
    pub lobby_id: u32,

    pub created: Timestamp,

    pub lobby_size: u8,

    pub region: Region,

    pub refinement: RelicRefinement,

    pub activity: String,

    pub rotation_type: RotationType,

    pub amount_players: u8,

    pub dummies: u8,
}

#[spacetimedb::table(accessor = lobby_join, public)]
pub struct LobbyJoin {
    #[primary_key]
    pub user_id: u32,

    #[index(btree)]
    pub lobby_id: u32,
}

#[spacetimedb::table(accessor = lobby_ban)]
pub struct LobbyBan {
    #[index(btree)]
    pub lobby_id: u32,

    #[index(btree)]
    pub user_id: u32,
}

#[spacetimedb::table(accessor = allowlist)]
pub struct AllowList {
    #[primary_key]
    pub id: Identity,
}
