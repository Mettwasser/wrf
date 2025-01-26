use std::{
    borrow::Cow,
    iter::{
        once,
        Once,
    },
};

use async_trait::async_trait;
use axum::{
    Extension,
    Router as AxumRouter,
};
use loco_rs::{
    app::{
        AppContext,
        Initializer,
    },
    controller::middleware::auth,
    Result,
};
use serde::Deserialize;
use socketioxide::extract::{
    Data,
    SocketRef,
    State,
};
use socketioxide_core::adapter::{
    Room,
    RoomParam,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::models::users;

#[derive(Clone)]
struct SocketState {
    ctx: AppContext,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum SubscriptionType {
    Recent,
    Lobby { id: i32 },
}

impl RoomParam for SubscriptionType {
    type IntoIter = Once<Room>;
    fn into_room_iter(self) -> Self::IntoIter {
        once(match self {
            SubscriptionType::Recent => Room::Borrowed("recent"),
            SubscriptionType::Lobby { id } => Room::Owned(format!("lobby:{}", id)),
        })
    }
}

#[derive(Debug)]
enum ClientEvent {
    Subscribe,
    Unsubscribe,
}

impl From<ClientEvent> for Cow<'static, str> {
    fn from(val: ClientEvent) -> Self {
        match val {
            ClientEvent::Subscribe => Cow::Borrowed("subscribe"),
            ClientEvent::Unsubscribe => Cow::Borrowed("unsubscribe"),
        }
    }
}

#[derive(Debug)]
pub enum ServerEvent {
    CreateLobby,
    PlayerCountUpdate,
}

impl AsRef<str> for ServerEvent {
    fn as_ref(&self) -> &str {
        match self {
            ServerEvent::CreateLobby => "create-lobby",
            ServerEvent::PlayerCountUpdate => "player-count-update",
        }
    }
}

fn on_subscribe(socket: SocketRef, Data(data): Data<SubscriptionType>) {
    info!("Incoming subscription: {:?}", data);
    socket.join(data);
}

fn on_unsubscribe(socket: SocketRef, Data(data): Data<SubscriptionType>) {
    info!("Incoming unsubscribe: {:?}", data);
    socket.leave(data);
}

async fn on_connect(socket: SocketRef, State(state): State<SocketState>) {
    let res = auth::extract_jwt_from_request_parts(socket.req_parts(), &state.ctx);
    if res.is_err() {
        socket.disconnect().ok();
        return;
    };

    let res = users::Model::find_by_pid(&state.ctx.db, &res.unwrap().claims.pid).await;
    if res.is_err() {
        socket.disconnect().ok();
        return;
    };

    socket.on(ClientEvent::Subscribe, on_subscribe);
    socket.on(ClientEvent::Unsubscribe, on_unsubscribe);
}

pub struct SocketInitializer;

#[async_trait]
impl Initializer for SocketInitializer {
    fn name(&self) -> String {
        "socket".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let (layer, io) = socketioxide::SocketIo::builder()
            .with_state(SocketState { ctx: ctx.clone() })
            .build_layer();

        io.ns("/", on_connect);

        Ok(router
            .layer(
                ServiceBuilder::new()
                    .layer(CorsLayer::very_permissive())
                    .layer(layer),
            )
            .layer(Extension(io)))
    }
}
