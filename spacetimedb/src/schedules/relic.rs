use std::collections::HashMap;

use serde::Deserialize;
use spacetimedb::{
    ProcedureContext,
    Table,
};

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct ExportRelic {
    pub category: String,
    pub era: String,
}

#[spacetimedb::table(accessor = relic)]
pub struct Relic {
    #[primary_key]
    pub relic: String,
}

#[spacetimedb::table(accessor = relic_timer, scheduled(refresh))]
pub struct RelicTimer {
    #[primary_key]
    #[auto_inc]
    pub scheduled_id: u64,
    pub scheduled_at: spacetimedb::ScheduleAt,
}

#[spacetimedb::procedure]
pub fn refresh(ctx: &mut ProcedureContext, _timer: RelicTimer) -> Result<(), String> {
    let resp = match ctx
        .http
        .get("https://raw.githubusercontent.com/calamity-inc/warframe-public-export-plus/refs/heads/senpai/ExportRelics.json")
    {
        Ok(resp) => resp.into_body().into_string_lossy(),
        Err(e) => {
            let err = e.to_string();
            log::error!("{err}");
            return Err(Error::Other(err).into());
        }
    };

    let relics = match serde_json::from_str::<HashMap<String, ExportRelic>>(&resp) {
        Ok(root) => root
            .into_values()
            .map(|ExportRelic { category, era }| format!("{era} {category}"))
            .collect::<Vec<_>>(),

        Err(e) => {
            let err = e.to_string();
            log::error!("{err}");
            return Err(Error::Other(err).into());
        }
    };

    ctx.with_tx(|ctx| {
        for relic in &relics {
            let _ = ctx.db.relic().try_insert(Relic {
                relic: relic.to_owned(),
            });
        }
    });

    Ok(())
}
