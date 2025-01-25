use axum::Router as AxumRouter;
use chrono::Local;
use loco_rs::{
    app::Initializer,
    prelude::*,
};
use tracing::{
    error,
    info,
};

use crate::models::_entities::{
    lobbies,
    register_sessions,
};

#[derive(Debug)]
pub struct DeleteExpiredDbEntries;

#[async_trait]
impl Initializer for DeleteExpiredDbEntries {
    fn name(&self) -> String {
        "delete_expired_db_entries".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let ctx = ctx.clone();
        tokio::task::spawn(async move {
            loop {
                info!("Deleting expired sessions...");
                let result = register_sessions::Entity::delete_many()
                    .filter(register_sessions::Column::Expiry.lt(Local::now().naive_local()))
                    .exec(&ctx.db)
                    .await;

                if let Err(err) = result {
                    error!("Failed to delete expired register sessions: {:?}", err);
                }

                let result = lobbies::Entity::delete_many()
                    .filter(lobbies::Column::Expiry.lt(Local::now().naive_local()))
                    .exec(&ctx.db)
                    .await;

                if let Err(err) = result {
                    error!("Failed to delete expired lobbies: {:?}", err);
                }

                info!("Done deleting expired sessions!");

                tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
            }
        });

        Ok(router)
    }
}
