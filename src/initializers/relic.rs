use std::{
    collections::{
        HashMap,
        HashSet,
    },
    sync::Arc,
};

use async_trait::async_trait;
use axum::{
    Extension,
    Router as AxumRouter,
};
use derive_more::derive::Display;
use loco_rs::{
    self,
    app::{
        AppContext,
        Initializer,
    },
    Result,
};
use serde::{
    Deserialize,
    Serialize,
};

pub type RelicItem = HashMap<String, RelicItemValue>;

#[derive(Serialize, Deserialize, Display, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[display("{} {}", era, category)]
pub struct RelicItemValue {
    category: String,

    era: Era,
}

#[derive(Serialize, Deserialize, Display, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Era {
    Axi,
    Lith,
    Meso,
    Neo,
    Requiem,
}

pub struct RelicInitializer;

#[async_trait]
impl Initializer for RelicInitializer {
    fn name(&self) -> String {
        "relic".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let relic_items = reqwest::get("https://raw.githubusercontent.com/calamity-inc/warframe-public-export-plus/refs/heads/senpai/ExportRelics.json")
            .await
            .map_err(|err| loco_rs::Error::Any(err.into()))?
            .json::<RelicItem>()
            .await
            .map_err(|err| loco_rs::Error::Any(err.into()))?;

        let relics = relic_items
            .into_values()
            .map(|v| v.to_string())
            .collect::<HashSet<String>>();

        tracing::info!(?relics);

        Ok(router.layer(Extension(Arc::new(relics))))
    }
}
