use loco_rs::prelude::*;
use sea_orm::{
    prelude::DateTimeWithTimeZone,
    EntityName,
    FromQueryResult,
};
use serde::Serialize;

use super::auth::UserResponse;
use crate::models::{
    _entities::{
        lobbies,
        sea_orm_active_enums::{
            Region,
            RelicRefinement,
        },
        users_lobbies,
    },
    users,
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyResponse {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub id: i32,
    pub expiry: DateTime,
    pub region: Region,
    pub refinement: RelicRefinement,
    pub activity: String,
    pub size: i32,
    pub user_id: i32,
}

impl LobbyResponse {
    pub fn new(
        lobbies::Model {
            created_at,
            updated_at,
            id,
            expiry,
            region,
            refinement,
            activity,
            size,
            user_id,
        }: lobbies::Model,
    ) -> Self {
        Self {
            created_at,
            updated_at,
            id,
            expiry,
            region,
            refinement,
            activity,
            size,
            user_id,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LobbyAndUserResponse {
    pub user: UserResponse,
    pub lobby: LobbyResponse,
}

impl LobbyAndUserResponse {
    pub fn new(user: users::Model, lobby: lobbies::Model) -> Self {
        Self {
            lobby: LobbyResponse::new(lobby),
            user: UserResponse::new(user),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentLobby {
    lobby: LobbyResponse,
    host: UserResponse,
    players: i32,
}

impl FromQueryResult for RecentLobby {
    fn from_query_result(res: &sea_orm::QueryResult, _pre: &str) -> Result<Self, sea_orm::DbErr> {
        let lobby = lobbies::Model::from_query_result(res, lobbies::Entity.table_name())?.into();
        let host = users::Model::from_query_result(res, users::Entity.table_name())?;
        let joined_players =
            users_lobbies::Model::from_query_result(res, users_lobbies::Entity.table_name())?;

        Ok(Self {
            lobby: LobbyResponse::new(lobby),
            host: UserResponse::new(host),
            players: 1,
        })
    }
}
