use spacetimedb::{
    SpacetimeType,
    ViewContext,
};

use crate::{
    model::{
        LobbyBan,
        User,
        UserDetails,
        lobby_ban__view,
        user__view,
        user_details__view,
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

#[derive(SpacetimeType)]
pub struct Me {
    pub user: User,
    pub details: UserDetails,
}

#[spacetimedb::view(accessor = me, public)]
fn me(ctx: &ViewContext) -> Option<Me> {
    ctx.db.user().id().find(ctx.user_id()?).and_then(|user| {
        Some(Me {
            details: ctx.db.user_details().user_id().find(user.id)?,
            user,
        })
    })
}

#[spacetimedb::view(accessor = my_verify_timer, public)]
fn my_verify_timer(ctx: &ViewContext) -> Option<VerifyTimer> {
    ctx.db.verify_timer().user_id().find(ctx.user_id()?)
}

#[spacetimedb::view(accessor = my_bans, public)]
fn my_bans(ctx: &ViewContext) -> Vec<LobbyBan> {
    ctx.db
        .lobby_ban()
        .user_id()
        .filter(ctx.user_id().unwrap_or(0))
        .collect()
}
