#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20241231_210221_lobbies;
mod m20241231_210807_users_lobbies;
mod m20241231_215132_add_lobbies_region;
mod m20250101_112311_add_lobbies_activity_refinement;
mod m20250101_194917_register_sessions;
mod m20250105_194858_add_lobbies_size;
mod m20250111_143735_add_user_ref_to_lobbies;
mod m20250111_152320_lobby_bans;
mod m20250126_115216_exceptional_refinement;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20241231_210221_lobbies::Migration),
            Box::new(m20241231_210807_users_lobbies::Migration),
            Box::new(m20241231_215132_add_lobbies_region::Migration),
            Box::new(m20250101_112311_add_lobbies_activity_refinement::Migration),
            Box::new(m20250101_194917_register_sessions::Migration),
            Box::new(m20250105_194858_add_lobbies_size::Migration),
            Box::new(m20250111_143735_add_user_ref_to_lobbies::Migration),
            Box::new(m20250111_152320_lobby_bans::Migration),
            Box::new(m20250126_115216_exceptional_refinement::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}