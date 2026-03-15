pub mod model;
pub mod schedules;
pub mod types;
pub mod utils;
pub mod views;

use std::time::Duration;

use spacetimedb::{
    Identity,
    ReducerContext,
    ScheduleAt,
    StdbRng,
    Table,
    TimeDuration,
    rand::{
        Rng,
        distributions::Alphanumeric,
    },
};

use crate::{
    model::{
        Lobby,
        LobbyBan,
        LobbyJoin,
        User,
        UserWarframeId,
        lobby,
        lobby_ban,
        lobby_join,
        user,
        user_warframe_id,
    },
    schedules::{
        disconnect_timer::{
            DisconnectTimer,
            disconnect_timer,
        },
        relic::{
            RelicTimer,
            relic,
            relic_timer,
        },
        verifier::{
            RETRY_OFFSET_TIME,
            VerifyTimer,
            verify_timer,
        },
    },
    types::{
        Region,
        RelicRefinement,
        RotationType,
    },
    utils::{
        ErrorToString,
        MapUniqueViolation,
        lobby_cleanup,
        remove_player_from_lobby,
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
    log::info!("Initializing...");

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
                ctx.timestamp + TimeDuration::from_duration(Duration::from_secs(30)),
            ),
        })?;

    log::info!("Initialization done!");

    Ok(())
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) -> Result<(), String> {
    log::info!("New connection");
    let auth_ctx = ctx.sender_auth();

    let Some((subject, identity)) = auth_ctx
        .jwt()
        .map(|claims| (claims.subject(), claims.identity()))
    else {
        return Err("Client connected without JWT".to_string());
    };

    ctx.db.disconnect_timer().user().delete(ctx.sender());

    log::info!("subject: {subject}, identity: {identity}");
    Ok(())
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) -> Result<(), String> {
    if ctx.db.lobby_join().user().find(ctx.sender()).is_some() {
        ctx.db.disconnect_timer().try_insert(DisconnectTimer {
            scheduled_id: 0,
            scheduled_at: ScheduleAt::Time(
                ctx.timestamp + TimeDuration::from_duration(Duration::from_mins(3)),
            ),
            user: ctx.sender(),
        })?;
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn set_username(ctx: &ReducerContext, name: String) -> Result<(), String> {
    let existing_user = ctx.db.user().id().find(ctx.sender());

    log::info!("{} is trying to set their username to {name}", ctx.sender());

    ctx.db
        .user()
        .id()
        .try_insert_or_update(User {
            id: ctx.sender(),
            username: name,
            verified: false,
            is_admin: existing_user.map(|u| u.is_admin).unwrap_or(false),
        })
        .map_unique_violation(|_| "username already taken")?;

    Ok(())
}

#[spacetimedb::reducer]
pub fn set_warframe_id(ctx: &ReducerContext, id: String) -> Result<(), String> {
    ctx.db
        .user_warframe_id()
        .user_id()
        .try_insert_or_update(UserWarframeId {
            user_id: ctx.sender(),
            warframe_id: id,
        })
        .error_as_string()?;

    ctx.db
        .verify_timer()
        .user_id()
        .try_insert_or_update(VerifyTimer {
            scheduled_id: 0,
            scheduled_at: ScheduleAt::Time(ctx.timestamp + RETRY_OFFSET_TIME),
            user_id: ctx.sender(),
            code: generate_random_code(ctx.rng()),
            attempts: 0,
        })
        .error_as_string()?;

    Ok(())
}

#[spacetimedb::reducer]
pub fn create_or_update_lobby(
    ctx: &ReducerContext,
    lobby_size: u8,
    region: Region,
    refinement: RelicRefinement,
    rotation_type: RotationType,
    activity: String,
) -> Result<(), String> {
    if ctx.db.user().id().find(ctx.sender()).is_none() {
        return Err("You haven't created a user yet!".to_owned());
    }

    if ctx.db.relic().relic().find(&activity).is_none() {
        return Err("Invalid relic".to_owned());
    }

    if !(2..=4).contains(&lobby_size) {
        return Err("Invalid lobby size".to_owned());
    }

    ctx.db
        .lobby()
        .host()
        .try_insert_or_update(Lobby {
            host: ctx.sender(),
            created: ctx.timestamp,
            activity,
            refinement,
            region,
            rotation_type,
            lobby_size,
            amount_players: 1,
            dummies: 0,
        })
        .map_unique_violation(|_| "You already have an open lobby")?;

    ctx.db
        .lobby_join()
        .try_insert(LobbyJoin {
            host: ctx.sender(),
            user: ctx.sender(),
        })
        .map_unique_violation(|_| "You can't join while having an opened lobby")?;

    Ok(())
}

#[spacetimedb::reducer]
pub fn join_lobby(ctx: &ReducerContext, lobby_id: Identity) -> Result<(), String> {
    let Some(mut lobby) = ctx.db.lobby().host().find(lobby_id) else {
        return Err("Lobby not found".to_owned());
    };

    let mut lobbies_user_is_banned_in = ctx.db.lobby_ban().user().filter(ctx.sender());

    if lobbies_user_is_banned_in.any(|lobby| lobby.host == lobby_id) {
        return Err("You are banned in this lobby".to_owned());
    }

    if lobby.amount_players == 4 {
        return Err("Lobby is full".to_owned());
    }

    ctx.db
        .lobby_join()
        .try_insert(LobbyJoin {
            host: lobby_id,
            user: ctx.sender(),
        })
        .map_unique_violation(|_| "You can't join multiple lobbies")?;

    lobby.amount_players += 1;

    ctx.db.lobby().host().update(lobby);

    Ok(())
}

#[spacetimedb::reducer]
pub fn leave_lobby(ctx: &ReducerContext) -> Result<(), String> {
    lobby_cleanup(&ctx.db, ctx.sender());

    Ok(())
}

#[spacetimedb::reducer]
pub fn kick(ctx: &ReducerContext, user: Identity) -> Result<(), String> {
    remove_player_from_lobby(&ctx.db, user);

    Ok(())
}

#[spacetimedb::reducer]
pub fn ban(ctx: &ReducerContext, user: Identity) -> Result<(), String> {
    ctx.db
        .lobby_ban()
        .try_insert(LobbyBan {
            host: ctx.sender(),
            user,
        })
        .ok();

    remove_player_from_lobby(&ctx.db, user);

    Ok(())
}

#[spacetimedb::reducer]
pub fn delete_my_account(ctx: &ReducerContext) -> Result<(), String> {
    log::info!("User {} is deleting their account", ctx.sender());

    if ctx.db.user().id().delete(ctx.sender()) {
        ctx.db.user_warframe_id().user_id().delete(ctx.sender());
        ctx.db.verify_timer().user_id().delete(ctx.sender());
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn add_dummy(ctx: &ReducerContext) -> Result<(), String> {
    if let Some(mut lobby) = ctx.db.lobby().host().find(ctx.sender())
        && lobby.dummies < 3
        && lobby.amount_players < 4
    {
        log::info!("User {} is adding a dummy", ctx.sender());
        lobby.dummies += 1;
        lobby.amount_players += 1;
        ctx.db.lobby().host().update(lobby);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn remove_dummy(ctx: &ReducerContext) -> Result<(), String> {
    if let Some(mut lobby) = ctx.db.lobby().host().find(ctx.sender())
        && lobby.dummies > 0
        // 1 because of the host themself
        && lobby.amount_players > 1
    {
        log::info!("User {} is removing a dummy", ctx.sender());
        lobby.dummies -= 1;
        lobby.amount_players -= 1;
        ctx.db.lobby().host().update(lobby);
    }

    Ok(())
}
