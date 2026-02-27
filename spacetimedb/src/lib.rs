pub mod model;
pub mod types;

use spacetimedb::{
    rand::{
        distributions::Alphanumeric,
        Rng,
    },
    ReducerContext,
    StdbRng,
    Table,
};

use crate::model::{
    user,
    user_verification,
    User,
    UserVerification,
};

fn generate_random_code(rng: &StdbRng) -> String {
    rng.sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) -> Result<(), String> {
    log::info!("New connection");
    let auth_ctx = ctx.sender_auth();

    let Some((subject, issuer)) = auth_ctx
        .jwt()
        .map(|claims| (claims.subject(), claims.issuer()))
    else {
        return Err("Client connected without JWT".to_string());
    };

    log::info!("sub: {}, iss: {}", subject, issuer);
    Ok(())
}

#[spacetimedb::reducer]
pub fn set_username(ctx: &ReducerContext, name: String) -> Result<(), String> {
    if let Some(mut user) = ctx.db.user().id().find(ctx.sender()) {
        user.username = name;
        user.verified = false;

        ctx.db.user().id().update(user);

        return Ok(());
    }

    ctx.db.user().insert(User {
        id: ctx.identity(),
        username: name,
        verified: false,
    });

    ctx.db.user_verification().insert(UserVerification {
        id: ctx.identity(),
        code: generate_random_code(ctx.rng()),
        warframe_id: None,
    });

    Ok(())
}
