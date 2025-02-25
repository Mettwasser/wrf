#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::Extension;
use chrono::{
    Duration,
    Local,
};
use loco_rs::prelude::*;
use migration::Alias;
use sea_orm::{
    JoinType,
    QueryOrder,
    QuerySelect,
    QueryTrait,
    RelationTrait,
};
use serde::Deserialize;
use socketioxide::SocketIo;

use crate::{
    initializers::socket::{
        ServerEvent,
        SubscriptionType,
    },
    models::{
        _entities::{
            lobbies::{
                self,
                ActiveModel,
                Column,
                Entity,
                Model,
            },
            sea_orm_active_enums::{
                Region,
                RelicRefinement,
            },
            users_lobbies,
        },
        users,
        Prefixer,
    },
    prelude::*,
    views::lobbies::{
        LobbyAndUserResponse,
        RecentLobby,
    },
    Relics,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyCreateParams {
    pub activity: String,
    pub region: Region,
    pub refinement: RelicRefinement,
    pub lobby_size: i32,
}

pub async fn create(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Extension(relics): Extension<Relics>,
    Extension(io): Extension<SocketIo>,
    Json(params): Json<LobbyCreateParams>,
) -> Result<Response> {
    let user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    if !(*relics).contains(&params.activity) {
        return bad_request_v("Invalid relic!");
    }

    let existing_lobby = Entity::find()
        .filter(Column::UserId.eq(user.id))
        .order_by_desc(Column::Expiry)
        .one(&ctx.db)
        .await?;

    if let Some(lobby) = existing_lobby {
        if lobby.expiry > Local::now().naive_local() {
            return conflict_v("You already have an active lobby!");
        }
    }

    let lobby = ActiveModel {
        user_id: Set(user.id),
        activity: Set(params.activity),
        region: Set(params.region),
        refinement: Set(params.refinement),
        size: Set(params.lobby_size),
        expiry: Set(Local::now().naive_local() + Duration::hours(3)),
        ..Default::default()
    };

    let lobby = lobby.insert(&ctx.db).await?;

    let response = LobbyAndUserResponse::new(user, lobby);

    io.to(SubscriptionType::Recent)
        .emit(ServerEvent::CreateLobby, &response)
        .await
        .ok();

    format::json(response)
}

pub async fn create_mock(Extension(io): Extension<SocketIo>) -> Result<Response> {
    let now = Local::now();

    let lobby = Model {
        id: 1,
        user_id: 1,
        activity: "Axi My Ass".to_string(),
        region: Region::EU,
        refinement: RelicRefinement::Radiant,
        size: 4,
        expiry: Local::now().naive_local() + Duration::hours(3),
        created_at: now.fixed_offset(),
        updated_at: now.fixed_offset(),
    };

    let user = users::Model {
        created_at: Local::now().fixed_offset(),
        updated_at: Local::now().fixed_offset(),
        id: 187,
        pid: Uuid::new_v4(),
        email: "".to_owned(),
        password: "".to_owned(),
        api_key: "".to_owned(),
        name: "Your Dad".to_owned(),
        reset_token: None,
        reset_sent_at: None,
        email_verification_token: None,
        email_verification_sent_at: None,
        email_verified_at: None,
    };

    io.to(SubscriptionType::Recent)
        .emit(
            ServerEvent::CreateLobby,
            &LobbyAndUserResponse::new(user, lobby),
        )
        .await
        .ok();

    format::empty()
}

pub async fn recent(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let _user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let select = lobbies::Entity::find()
        .limit(20)
        .order_by_desc(lobbies::Column::Expiry);

    let result = Prefixer::new(select)
        .add_columns(lobbies::Entity)
        .add_columns(users::Entity)
        .add_columns(users_lobbies::Entity)
        .selector
        .join(JoinType::LeftJoin, lobbies::Relation::User.def())
        .join(JoinType::LeftJoin, lobbies::Relation::UsersLobbies.def())
        .into_model::<RecentLobby>()
        .all(&ctx.db)
        .await?;

    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/lobbies/")
        .add("/", post(create))
        .add("/mock", post(create_mock))
}