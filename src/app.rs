use std::{
    env,
    path::Path,
    sync::Arc,
};

use async_trait::async_trait;
use axum::{
    http::HeaderValue,
    Extension,
};
use loco_rs::{
    app::{
        AppContext,
        Hooks,
        Initializer,
    },
    bgworker::Queue,
    boot::{
        create_app,
        BootResult,
        StartMode,
    },
    controller::AppRoutes,
    db::{
        self,
        truncate_table,
    },
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;
use tower_http::cors::CorsLayer;

use crate::{
    controllers,
    initializers,
    models::_entities::users,
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub captcha_secret: Arc<str>,
}

pub struct App;

#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![Box::new(
            initializers::view_engine::ViewEngineInitializer,
        )])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::register_sessions::routes())
            .add_route(controllers::lobbies::routes())
            .add_route(controllers::auth::routes())
    }
    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }
    fn register_tasks(_tasks: &mut Tasks) {
        // tasks-inject (do not remove)
    }
    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, users::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        Ok(())
    }

    async fn after_routes(router: axum::Router, _ctx: &AppContext) -> Result<axum::Router> {
        let appstate = AppState {
            captcha_secret: env::var("TURNSTILE_SECRET")
                .expect("you fucked up the config")
                .into(),
        };

        Ok(router
            .layer(Extension(appstate))
            .layer(CorsLayer::very_permissive()))
    }
}
