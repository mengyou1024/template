//! Database migration binary entry point.

use migration::Migrator;
use sea_orm::Database;
use sea_orm_migration::prelude::*;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_timer(LocalTime::new(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"
        )))
        .pretty()
        .init();

    let database_url = std::env::var("APP_DATABASE_URL").expect("APP_DATABASE_URL must be set");

    info!("Connecting to database...");
    let connection = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    info!("Running migrations...");
    Migrator::up(&connection, None)
        .await
        .expect("Failed to run migrations");

    info!("Migrations completed successfully!");
}
