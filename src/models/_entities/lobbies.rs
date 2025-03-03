//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.3

use sea_orm::entity::prelude::*;
use serde::{
    Deserialize,
    Serialize,
};

use super::sea_orm_active_enums::{
    Region,
    RelicRefinement,
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "lobbies")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i32,
    pub expiry: DateTime,
    pub region: Region,
    pub refinement: RelicRefinement,
    pub activity: String,
    pub size: i32,
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::lobby_bans::Entity")]
    LobbyBans,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(has_many = "super::users_lobbies::Entity")]
    UsersLobbies,
}

impl Related<super::lobby_bans::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LobbyBans.def()
    }
}

impl Related<super::users_lobbies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UsersLobbies.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}
