#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::Extension;
use chrono::{
    Duration,
    Local,
};
use loco_rs::prelude::*;
use sea_orm::QueryOrder;
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
                ActiveModel,
                Column,
                Entity,
                Model,
            },
            sea_orm_active_enums::{
                Regions,
                RelicRefinement,
            },
        },
        users,
    },
    prelude::*,
    Relics,
};

#[derive(Debug, Deserialize)]
pub struct LobbyCreateParams {
    pub activity: String,
    pub region: Regions,
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

    if !relics.contains(&params.activity) {
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

    io.to(SubscriptionType::Recent)
        .emit(ServerEvent::CreateLobby, &lobby)
        .await
        .ok();

    format::json(lobby)
}

pub async fn create_mock(Extension(io): Extension<SocketIo>) -> Result<Response> {
    let now = Local::now();
    io.to(SubscriptionType::Recent)
        .emit(
            ServerEvent::CreateLobby,
            &Model {
                id: 1,
                user_id: 1,
                activity: "Axi My Ass".to_string(),
                region: Regions::EU,
                refinement: RelicRefinement::Radiant,
                size: 4,
                expiry: Local::now().naive_local() + Duration::hours(3),
                created_at: now.fixed_offset(),
                updated_at: now.fixed_offset(),
            },
        )
        .await
        .ok();

    format::empty()
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/lobbies/")
        .add("/", post(create))
        .add("/mock", post(create_mock))
}

// next up: check if the
