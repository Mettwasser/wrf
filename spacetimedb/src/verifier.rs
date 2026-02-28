use std::time::Duration;

use serde::Deserialize;
use spacetimedb::{
    Identity,
    ProcedureContext,
    Table,
    TimeDuration,
    Timestamp,
};

use crate::{
    model::{
        user,
        user_verification,
    },
    utils::ErrorToString,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WarframeProfileRoot {
    results: Vec<WarframeProfileResult>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WarframeProfileResult {
    load_out_preset: LoadOutPreset,
}

#[derive(Debug, Deserialize)]
struct LoadOutPreset {
    #[serde(rename = "n")]
    name: String,
}

struct FilteredVerification {
    pub id: Identity,
    pub code: String,
    pub warframe_id: String,
}

#[spacetimedb::table(accessor = verify_timer, scheduled(verify))]
pub struct VerifyTimer {
    #[primary_key]
    #[auto_inc]
    pub scheduled_id: u64,
    pub scheduled_at: spacetimedb::ScheduleAt,
}

#[spacetimedb::procedure]
pub fn verify(ctx: &mut ProcedureContext, _timer: VerifyTimer) -> Result<(), String> {
    let verifications = ctx.with_tx(|ctx| {
        ctx.db
            .user()
            .iter()
            .filter(|u| !u.verified)
            .flat_map(|u| ctx.db.user_verification().id().find(u.id))
            .filter_map(|v| {
                v.warframe_id.map(|warframe_id| FilteredVerification {
                    id: v.id,
                    code: v.code,
                    warframe_id,
                })
            })
            .collect::<Vec<_>>()
    });

    log::info!("Verifying {} players", verifications.len());

    for verification in verifications {
        ctx.sleep_until(Timestamp::now() + TimeDuration::from_duration(Duration::from_secs(10)));
        let resp = match ctx
            .http
            .get(format!(
                "https://api.warframe.com/cdn/getProfileViewingData.php?playerId={}",
                verification.warframe_id
            ))
            .error_as_string()
        {
            Ok(resp) => resp.into_body().into_string_lossy(),
            Err(e) => {
                log::error!("{e}");
                continue;
            }
        };

        let loadout_name = match serde_json::from_str::<WarframeProfileRoot>(&resp) {
            Ok(root) => {
                if let Some(loadout) = root.results.into_iter().next() {
                    loadout.load_out_preset.name
                } else {
                    continue;
                }
            }
            Err(e) => {
                log::error!("{e}");
                continue;
            }
        };

        if loadout_name == verification.code {
            ctx.with_tx(|ctx| {
                if let Some(mut user) = ctx.db.user().id().find(verification.id) {
                    user.verified = true;
                    ctx.db.user().id().update(user);
                }
            })
        }
    }

    Ok(())
}
