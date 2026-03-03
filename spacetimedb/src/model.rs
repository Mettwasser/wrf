use spacetimedb::{
    Identity,
    Timestamp,
};

use crate::types::{
    Region,
    RelicRefinement,
    RotationType,
};

#[spacetimedb::table(accessor = user, public)]
pub struct User {
    #[primary_key]
    pub id: Identity,

    #[unique]
    pub username: String,

    pub verified: bool,

    pub is_admin: bool,
}

#[spacetimedb::table(accessor = user_verification)]
pub struct UserVerification {
    #[primary_key]
    pub id: Identity,
    pub code: String,
    pub warframe_id: Option<String>,
}

#[spacetimedb::table(accessor = lobby, public)]
pub struct Lobby {
    #[primary_key]
    pub host: Identity,

    pub created: Timestamp,

    pub lobby_size: u8,

    pub region: Region,

    pub refinement: RelicRefinement,

    pub activity: String,

    pub rotation_type: RotationType,

    pub amount_players: u8,
}

#[spacetimedb::table(accessor = lobby_join, public)]
pub struct LobbyJoin {
    #[primary_key]
    pub user: Identity,

    #[index(btree)]
    pub host: Identity,
}

#[spacetimedb::table(accessor = lobby_ban)]
pub struct LobbyBan {
    #[index(btree)]
    pub host: Identity,

    #[index(btree)]
    pub user: Identity,
}
