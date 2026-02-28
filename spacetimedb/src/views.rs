use spacetimedb::ViewContext;

use crate::model::{
    user__view,
    user_verification__view,
    User,
    UserVerification,
};

#[spacetimedb::view(accessor = verification, public)]
fn verification(ctx: &ViewContext) -> Option<UserVerification> {
    ctx.db.user_verification().id().find(ctx.sender())
}

#[spacetimedb::view(accessor = me, public)]
fn me(ctx: &ViewContext) -> Option<User> {
    ctx.db.user().id().find(ctx.sender())
}
