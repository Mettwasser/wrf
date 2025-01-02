use loco_rs::cli;
use migration::Migrator;
use wrf::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    dotenv::dotenv().expect("Failed to load .env");
    cli::main::<App, Migrator>().await
}
