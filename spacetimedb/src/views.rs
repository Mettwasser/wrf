use spacetimedb::{
    Query,
    ViewContext,
};

use crate::{
    model::{
        LobbyBan,
        User,
        lobby_ban__query,
        user__view,
    },
    schedules::verifier::{
        VerifyTimer,
        verify_timer__view,
    },
    utils::UserCtx,
};

#[spacetimedb::view(accessor = verification, public)]
fn verification(ctx: &ViewContext) -> Option<VerifyTimer> {
    ctx.db.verify_timer().user_id().find(ctx.user_id()?)
}

#[spacetimedb::view(accessor = me, public)]
fn me(ctx: &ViewContext) -> Option<User> {
    ctx.db.user().id().find(ctx.user_id()?)
}

#[spacetimedb::view(accessor = my_verify_timer, public)]
fn my_verify_timer(ctx: &ViewContext) -> Option<VerifyTimer> {
    ctx.db.verify_timer().user_id().find(ctx.user_id()?)
}

#[spacetimedb::view(accessor = my_bans, public)]
fn my_bans(ctx: &ViewContext) -> impl Query<LobbyBan> {
    ctx.from
        .lobby_ban()
        .filter(|row| row.user_id.eq(ctx.user_id().unwrap_or(0)))
}
