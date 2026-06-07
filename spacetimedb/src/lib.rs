pub mod error;
pub mod model;
pub mod permissions;
pub mod schedules;
pub mod types;
pub mod user_flags;
pub mod utils;
pub mod views;

use std::time::Duration;

use spacetimedb::{
    Identity,
    ReducerContext,
    ScheduleAt,
    SpacetimeType,
    StdbRng,
    Table,
    TimeDuration,
    rand::{
        Rng,
        distributions::Alphanumeric,
    },
};

use crate::{
    error::Error,
    model::{
        AllowList,
        Lobby,
        LobbyBan,
        LobbyJoin,
        User,
        UserDetails,
        UserId,
        allowlist,
        lobby,
        lobby_ban,
        lobby_join,
        user,
        user_details,
        user_id,
    },
    permissions::Permissions,
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
    user_flags::UserFlags,
    utils::{
        MapUniqueViolation,
        PermissionsCtx,
        UserCtx,
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

/// # Panics
/// Never, as
#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) -> Result<(), String> {
    log::info!("Initializing...");

    log::info!("Initializing Relic fetcher");
    ctx.db
        .relic_timer()
        .scheduled_id()
        .try_insert_or_update(RelicTimer {
            scheduled_id: 0,
            scheduled_at: ScheduleAt::Interval(TimeDuration::from_duration(Duration::from_hours(
                72,
            ))),
        })?;

    ctx.db
        .relic_timer()
        .scheduled_id()
        .try_insert_or_update(RelicTimer {
            scheduled_id: 0,
            scheduled_at: ScheduleAt::Time(
                ctx.timestamp + TimeDuration::from_duration(Duration::from_secs(30)),
            ),
        })?;

    ctx.db.allowlist().insert(AllowList {
        id: Identity::from_hex("c200559e7f4e204caf92afd416b1f1f883ed23983ea5a7de21b6c7a17a2af88b")
            .unwrap(),
    });

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
        return Err(Error::MissingJwt.into());
    };

    if let Some(id) = ctx.user_id() {
        ctx.db.disconnect_timer().user_id().delete(id);
    } else {
        ctx.db.user_id().insert(UserId {
            identity: ctx.sender(),
            id: 0,
        });
    }

    log::info!("subject: {subject}, identity: {identity}");

    Ok(())
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) -> Result<(), String> {
    if let Some(user_id) = ctx.user_id()
        && ctx.db.lobby_join().user_id().find(user_id).is_some()
    {
        ctx.db.disconnect_timer().try_insert(DisconnectTimer {
            scheduled_id: 0,
            scheduled_at: ScheduleAt::Time(
                ctx.timestamp + TimeDuration::from_duration(Duration::from_mins(3)),
            ),
            user_id,
        })?;
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn set_username(ctx: &ReducerContext, name: String) -> Result<(), String> {
    log::info!("{} is trying to set their username to {name}", ctx.sender());

    if !(4..=24).contains(&name.len())
        || !name
            .chars()
            .all(|c| c.is_alphanumeric() || ['.', ',', '-', '_'].contains(&c))
    {
        log::error!("{name} does not meet the requirements");
        return Err(Error::InvalidUsername.into());
    }

    let user_id = ctx.user_id().unwrap_or(0);

    ctx.db
        .user()
        .id()
        .try_insert_or_update(User { id: user_id, name })
        .map_unique_violation(|_| Error::UsernameTaken)?;

    let existing_details = ctx.db.user_details().user_id().find(user_id);

    ctx.db
        .user_details()
        .user_id()
        .try_insert_or_update(UserDetails {
            user_id,
            flags: existing_details.map_or(UserFlags::default(), |details| details.flags)
                & !UserFlags::Verified,
            permissions: existing_details
                .map_or(Permissions::default(), |details| details.permissions),
        })?;

    log::info!("{} successfully set their username", ctx.sender());

    Ok(())
}

#[spacetimedb::reducer]
pub fn set_warframe_id(ctx: &ReducerContext, warframe_id: String) -> Result<(), String> {
    let user_id = ctx.user_id_or_err()?;

    ctx.db
        .verify_timer()
        .user_id()
        .try_insert_or_update(VerifyTimer {
            scheduled_id: 0,
            scheduled_at: ScheduleAt::Time(ctx.timestamp + RETRY_OFFSET_TIME),
            user_id,
            warframe_id,
            code: generate_random_code(ctx.rng()),
            attempts: 0,
        })?;

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
    ctx.require_permissions(Permissions::CREATE_LOBBY)?;

    let Some(user_id) = ctx.user_id() else {
        return Err(Error::UserNotCreated.into());
    };

    if ctx.db.relic().relic().find(&activity).is_none() {
        return Err(Error::InvalidRelic.into());
    }

    if !(2..=4).contains(&lobby_size) {
        return Err(Error::InvalidLobbySize.into());
    }

    ctx.db
        .lobby()
        .lobby_id()
        .try_insert_or_update(Lobby {
            lobby_id: user_id,
            created: ctx.timestamp,
            activity,
            refinement,
            region,
            rotation_type,
            lobby_size,
            amount_players: 1,
            dummies: 0,
        })
        .map_unique_violation(|_| Error::LobbyAlreadyOpened)?;

    ctx.db
        .lobby_join()
        .try_insert(LobbyJoin {
            lobby_id: user_id,
            user_id,
        })
        .map_unique_violation(|_| Error::CantJoinWhileHosting)?;

    Ok(())
}

#[spacetimedb::reducer]
pub fn join_lobby(ctx: &ReducerContext, lobby_id: u32) -> Result<(), String> {
    ctx.require_permissions(Permissions::JOIN_LOBBY)?;

    let Some(user_id) = ctx.user_id() else {
        return Err(Error::UserNotCreated.into());
    };

    let Some(mut lobby) = ctx.db.lobby().lobby_id().find(lobby_id) else {
        return Err(Error::LobbyNotFound.into());
    };

    let mut lobbies_user_is_banned_in = ctx.db.lobby_ban().user_id().filter(user_id);

    if lobbies_user_is_banned_in.any(|lobby_ban| lobby_ban.lobby_id == lobby_id) {
        return Err(Error::BannedFromLobby.into());
    }

    if lobby.amount_players == 4 {
        return Err(Error::LobbyFull.into());
    }

    ctx.db
        .lobby_join()
        .try_insert(LobbyJoin { lobby_id, user_id })
        .map_unique_violation(|_| Error::JoinMultipleLobbies)?;

    lobby.amount_players += 1;

    ctx.db.lobby().lobby_id().update(lobby);

    Ok(())
}

#[spacetimedb::reducer]
pub fn leave_lobby(ctx: &ReducerContext) -> Result<(), String> {
    lobby_cleanup(&ctx.db, ctx.user_id_or_err()?);

    Ok(())
}

#[spacetimedb::reducer]
pub fn kick(ctx: &ReducerContext, user: u32) -> Result<(), String> {
    remove_player_from_lobby(&ctx.db, user);

    Ok(())
}

#[spacetimedb::reducer]
pub fn ban(ctx: &ReducerContext, user: u32) -> Result<(), String> {
    ctx.db
        .lobby_ban()
        .try_insert(LobbyBan {
            lobby_id: ctx.user_id_or_err()?,
            user_id: user,
        })
        .ok();

    remove_player_from_lobby(&ctx.db, user);

    Ok(())
}

#[spacetimedb::reducer]
pub fn delete_my_account(ctx: &ReducerContext) -> Result<(), String> {
    let user_id = ctx.user_id_or_err()?;

    if ctx.db.user().id().delete(user_id) {
        // We don't delete the identity -> user
        // mapping here, because if the user comes back at some point
        // they would bump the ID again, which we don't want
        log::info!(
            "User {} ({}) is deleting their account",
            ctx.sender(),
            user_id
        );

        ctx.db.user_details().user_id().delete(user_id);
        ctx.db.verify_timer().user_id().delete(user_id);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn add_dummy(ctx: &ReducerContext) -> Result<(), String> {
    let user_id = ctx.user_id_or_err()?;

    if let Some(mut lobby) = ctx.db.lobby().lobby_id().find(user_id)
        && lobby.dummies < 3
        && lobby.amount_players < 4
    {
        log::info!("User {} is adding a dummy", ctx.sender());
        lobby.dummies += 1;
        lobby.amount_players += 1;
        ctx.db.lobby().lobby_id().update(lobby);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn remove_dummy(ctx: &ReducerContext) -> Result<(), String> {
    let user_id = ctx.user_id_or_err()?;

    if let Some(mut lobby) = ctx.db.lobby().lobby_id().find(user_id)
        && lobby.dummies > 0
        // 1 because of the host themself
        && lobby.amount_players > 1
    {
        log::info!("User {} is removing a dummy", ctx.sender());
        lobby.dummies -= 1;
        lobby.amount_players -= 1;
        ctx.db.lobby().lobby_id().update(lobby);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn edit_user_flags(ctx: &ReducerContext, target: u32, flags: UserFlags) -> Result<(), String> {
    ctx.require_permissions(Permissions::MANAGE_USER_FLAGS)?;

    if let Some(mut details) = ctx.db.user_details().user_id().find(target) {
        details.flags = flags;
        ctx.db.user_details().user_id().update(details);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn kickstart_perms(ctx: &ReducerContext, target: Identity) -> Result<(), String> {
    if ctx.db.allowlist().id().find(ctx.sender()).is_none() {
        return Ok(());
    }

    if let Some(user) = ctx.db.user_id().identity().find(target) {
        ctx.db
            .user_details()
            .user_id()
            .try_insert_or_update(UserDetails {
                user_id: user.id,
                permissions: Permissions::all_flags(),
                flags: UserFlags::Verified,
            })?;
    }

    Ok(())
}

#[derive(Debug, SpacetimeType)]
pub struct OldUser {
    id: Identity,
    username: String,
    verified: bool,
}

#[spacetimedb::reducer]
pub fn migrate_user_elevated(ctx: &ReducerContext, user: OldUser) -> Result<(), String> {
    if ctx.db.allowlist().id().find(ctx.sender()).is_none() {
        return Ok(());
    }

    let OldUser {
        id: identity,
        username: name,
        verified,
    } = user;

    let id = ctx.db.user_id().try_insert(UserId { id: 0, identity })?.id;

    ctx.db.user().try_insert(User { id, name })?;

    let mut details = UserDetails::with_default(id);

    if verified {
        details.flags |= UserFlags::Verified;
    } else {
        details.flags &= !UserFlags::Verified;
    }

    ctx.db.user_details().try_insert(details)?;

    Ok(())
}
