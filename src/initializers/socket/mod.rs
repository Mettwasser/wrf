use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
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
use rmpv::Value;
use socketioxide::{
    extract::{
        Data,
        SocketRef,
        State,
    },
    handler::ConnectMiddleware,
};

#[derive(Clone)]
struct SocketState {
    ctx: AppContext,
}

async fn on_connect(socket: SocketRef, State(state): State<SocketState>) {
    let mut req_parts = socket.req_parts().clone();
    let res = auth::JWT::from_request_parts(&mut req_parts, &state.ctx).await;
    if res.is_err() {
        socket.disconnect().ok();
        return;
    };
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

        io.ns("/ws", on_connect);

        Ok(router.layer(layer).layer(Extension(io)))
    }
}
