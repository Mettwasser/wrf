use std::time::Duration;

use serde::{
    Deserialize,
    Deserializer,
};
use spacetimedb::{
    Identity,
    ProcedureContext,
    Table,
    TimeDuration,
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

struct VerificationAndUser {
    pub id: Identity,
    pub code: String,
    pub warframe_id: String,
    pub username: String,
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
            .filter_map(|u| {
                let v = ctx.db.user_verification().id().find(u.id)?;

                v.warframe_id.map(|warframe_id| VerificationAndUser {
                    id: v.id,
                    code: v.code.clone(), // Assuming String/Clone is needed
                    warframe_id,
                    username: u.username.clone(),
                })
            })
            .collect::<Vec<_>>()
    });

    log::info!("Verifying {} players", verifications.len());

    for verification in verifications {
        ctx.sleep_until(ctx.timestamp + TimeDuration::from_duration(Duration::from_secs(3)));
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

        let profile = match serde_json::from_str::<WarframeProfileRoot>(&resp) {
            Ok(root) => {
                if let Some(profile) = root.results.into_iter().next() {
                    profile
                } else {
                    continue;
                }
            }
            Err(e) => {
                log::error!("{e}");
                continue;
            }
        };

        if profile.load_out_preset.name == verification.code
            && profile.display_name == verification.username
        {
            ctx.with_tx(|ctx| {
                if let Some(mut user) = ctx.db.user().id().find(verification.id) {
                    user.verified = true;
                    ctx.db.user().id().update(user);
                }
            });
            log::info!("verified {}", verification.username);
        }
    }

    Ok(())
}
