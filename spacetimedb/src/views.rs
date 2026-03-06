use spacetimedb::ViewContext;

use crate::{
    model::{
        User,
        UserWarframeId,
        user__view,
        user_warframe_id__view,
    },
    schedules::verifier::{
        VerifyTimer,
        verify_timer__view,
    },
};

#[spacetimedb::view(accessor = verification, public)]
fn verification(ctx: &ViewContext) -> Option<VerifyTimer> {
    ctx.db.verify_timer().user_id().find(ctx.sender())
}

#[spacetimedb::view(accessor = me, public)]
fn me(ctx: &ViewContext) -> Option<User> {
    ctx.db.user().id().find(ctx.sender())
}

#[spacetimedb::view(accessor = warframe_id, public)]
fn warframe_id(ctx: &ViewContext) -> Option<UserWarframeId> {
    ctx.db.user_warframe_id().user_id().find(ctx.sender())
}

#[spacetimedb::view(accessor = my_verify_timer, public)]
fn my_verify_timer(ctx: &ViewContext) -> Option<VerifyTimer> {
    ctx.db.verify_timer().user_id().find(ctx.sender())
}
