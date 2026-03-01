pub mod model;
pub mod schedules;
pub mod types;
pub mod utils;
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
        lobby,
        user,
        user_verification,
        Lobby,
        User,
        UserVerification,
    },
    schedules::{
        relic::{
            relic,
            relic_timer,
            RelicTimer,
        },
        verifier::{
            verify_timer,
            VerifyTimer,
        },
    },
    types::{
        Region,
        RelicRefinement,
        RotationType,
    },
    utils::MapUniqueViolation,
};

fn generate_random_code(rng: &StdbRng) -> String {
    rng.sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) -> Result<(), String> {
    log::info!("Initializing...");

    log::info!("Initializing Verifier");
    ctx.db
        .verify_timer()
        .scheduled_id()
        .try_insert_or_update(VerifyTimer {
            scheduled_id: 1,
            scheduled_at: ScheduleAt::Interval(TimeDuration::from_duration(Duration::from_mins(
                10,
            ))),
        })?;

    log::info!("Initializing Relic fetcher");
    ctx.db
        .relic_timer()
        .scheduled_id()
        .try_insert_or_update(RelicTimer {
            scheduled_id: 1,
            scheduled_at: ScheduleAt::Interval(TimeDuration::from_duration(Duration::from_hours(
                72,
            ))),
        })?;

    ctx.db
        .relic_timer()
        .scheduled_id()
        .try_insert_or_update(RelicTimer {
            scheduled_id: 2,
            scheduled_at: ScheduleAt::Time(
                ctx.timestamp + TimeDuration::from_duration(Duration::from_mins(1)),
            ),
        })?;

    log::info!("Initialization done!");

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
            is_admin: false,
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

#[spacetimedb::reducer]
pub fn create_lobby(
    ctx: &ReducerContext,
    space: u8,
    region: Region,
    refinement: RelicRefinement,
    rotation_type: RotationType,
    activity: String,
) -> Result<(), String> {
    if ctx.db.relic().relic().find(&activity).is_none() {
        return Err("Invalid relic".to_owned());
    }

    if !(2..=4).contains(&space) {
        return Err("Invalid lobby size".to_owned());
    }

    ctx.db
        .lobby()
        .try_insert(Lobby {
            host: ctx.sender(),
            created: ctx.timestamp,
            activity,
            refinement,
            region,
            rotation_type,
            space,
            amount_players: 1,
        })
        .map_unique_violation(|_| "You already opened a lobby")?;

    Ok(())
}
