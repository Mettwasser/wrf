pub mod model;
pub mod types;
pub mod utils;
pub mod verifier;
pub mod views;

use std::time::Duration;

use spacetimedb::{
    rand::{
        distributions::Alphanumeric,
        Rng,
    },
    ReducerContext,
    ScheduleAt,
    StdbRng,
    Table,
    TimeDuration,
};

use crate::{
    model::{
        user,
        user_verification,
        User,
        UserVerification,
    },
    utils::MapUniqueViolation,
    verifier::{
        verify_timer,
        VerifyTimer,
    },
};

fn generate_random_code(rng: &StdbRng) -> String {
    rng.sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) -> Result<(), String> {
    ctx.db.verify_timer().try_insert(VerifyTimer {
        scheduled_id: 0,
        scheduled_at: ScheduleAt::Interval(TimeDuration::from_duration(Duration::from_mins(10))),
    })?;

    Ok(())
}

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
    ctx.db
        .user()
        .id()
        .try_insert_or_update(User {
            id: ctx.sender(),
            username: name,
            verified: false,
        })
        .map_unique_violation(|_| "username already taken")?;

    if ctx.db.user_verification().id().find(ctx.sender()).is_none() {
        ctx.db.user_verification().insert(UserVerification {
            id: ctx.sender(),
            code: generate_random_code(ctx.rng()),
            warframe_id: None,
        });
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn set_warframe_id(ctx: &ReducerContext, id: String) -> Result<(), String> {
    let existing = ctx.db.user_verification().id().find(ctx.sender());

    let (code, id_to_set) = match existing {
        Some(record) => (record.code, Some(id)),
        None => (generate_random_code(ctx.rng()), Some(id)),
    };

    ctx.db
        .user_verification()
        .id()
        .try_insert_or_update(UserVerification {
            id: ctx.sender(),
            code,
            warframe_id: id_to_set,
        })
        .map_err(|e| format!("Failed to update Warframe ID: {}", e))?;

    Ok(())
}
