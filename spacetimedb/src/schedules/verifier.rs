use std::time::Duration;

use http::StatusCode;
use serde::{
    Deserialize,
    Deserializer,
};
use spacetimedb::{
    ProcedureContext,
    ScheduleAt,
    Table,
};

use crate::{
    error::Error,
    model::{
        user,
        user_details,
    },
    user_flags::UserFlags,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WarframeProfileRoot {
    results: Vec<WarframeProfileResult>,
}

fn clean_pua_chars<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // Filters out the BMP Private Use Area (U+E000 to U+F8FF)
    Ok(s.chars()
        .filter(|&c| !('\u{E000}'..='\u{F8FF}').contains(&c))
        .collect())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WarframeProfileResult {
    load_out_preset: LoadOutPreset,
    #[serde(deserialize_with = "clean_pua_chars")]
    display_name: String,
}

#[derive(Debug, Deserialize)]
struct LoadOutPreset {
    #[serde(rename = "n")]
    name: String,
}

#[spacetimedb::table(accessor = verify_timer, scheduled(verify))]
pub struct VerifyTimer {
    #[primary_key]
    #[auto_inc]
    pub scheduled_id: u64,
    pub scheduled_at: spacetimedb::ScheduleAt,

    #[unique]
    pub user_id: u32,
    pub warframe_id: String,
    pub code: String,
    pub attempts: u8,
}

fn retry_in(duration: Duration, ctx: &mut ProcedureContext, entry: VerifyTimer) {
    // why ts needs to be impl Fn ;-;
    ctx.with_tx(move |ctx| {
        let code = entry.code.clone();
        let warframe_id = entry.warframe_id.clone();

        ctx.db.verify_timer().insert(VerifyTimer {
            attempts: entry.attempts + 1,
            code,
            warframe_id,
            scheduled_at: ScheduleAt::Time(ctx.timestamp + duration),
            ..entry
        })
    });
}

const RETRY_LIMIT: u8 = 3;
pub const RETRY_OFFSET_TIME: Duration = Duration::from_mins(10);

fn limit_reached(entry: &VerifyTimer) -> bool {
    entry.attempts >= RETRY_LIMIT
}

/// This is a weird one.
#[spacetimedb::procedure]
pub fn verify(ctx: &mut ProcedureContext, entry: VerifyTimer) -> Result<(), String> {
    let Some(user) = ctx.with_tx(|ctx| ctx.db.user().id().find(entry.user_id)) else {
        return Ok(());
    };

    log::info!(
        "Verifying {} players for the {}. time",
        user.name,
        entry.attempts + 1
    );

    let resp = match ctx.http.get(format!(
        "https://api.warframe.com/cdn/getProfileViewingData.php?playerId={}",
        entry.warframe_id
    )) {
        Ok(resp) if resp.status() == StatusCode::OK => resp.into_body().into_string_lossy(),
        Ok(_) => {
            let wf_id = entry.warframe_id.clone();
            if !limit_reached(&entry) {
                retry_in(RETRY_OFFSET_TIME, ctx, entry);
            }

            log::error!("Warframe User with ID {wf_id} not found");
            return Err(Error::WarframeUserNotFound(wf_id).into());
        }
        Err(e) => {
            if !limit_reached(&entry) {
                retry_in(RETRY_OFFSET_TIME, ctx, entry);
            }

            let err = e.to_string();
            log::error!("{err}");
            return Err(Error::Other(err).into());
        }
    };

    log::info!("Sent verify request for {}", user.name);

    let profile = match serde_json::from_str::<WarframeProfileRoot>(&resp) {
        Ok(root) => {
            if let Some(profile) = root.results.into_iter().next() {
                profile
            } else {
                // profile not found; do not retry
                return Ok(());
            }
        }
        Err(e) => {
            // could be some weird issue with bytes not arriving correctly, idk
            // retry
            if !limit_reached(&entry) {
                retry_in(RETRY_OFFSET_TIME, ctx, entry);
            }

            let err = e.to_string();
            log::error!("{err}");
            return Err(Error::Other(err).into());
        }
    };

    log::info!("Deserialized payload for {}", user.name);

    log::info!("doing final check for {}", user.name);

    if profile.load_out_preset.name == entry.code && profile.display_name == user.name {
        ctx.with_tx(|ctx| {
            if let Some(mut user) = ctx.db.user_details().user_id().find(user.id) {
                user.flags |= UserFlags::Verified;
                ctx.db.user_details().user_id().update(user);
            }
        });

        log::info!("verified {}", user.name);
    } else {
        log::info!("verification failed for {}", user.name);

        if !limit_reached(&entry) {
            retry_in(RETRY_OFFSET_TIME, ctx, entry);
        }
    }

    Ok(())
}
