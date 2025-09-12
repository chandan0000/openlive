use std::time::Duration;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};

use tracing::info;
use tracing_subscriber::FmtSubscriber;

mod router;

pub async fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let mut opt = ConnectOptions::new("postgresql://postgres:12345@localhost:5432/mywellness");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = Database::connect(opt).await?;
    Migrator::up(&db, None).await?;

    let _ = tracing::subscriber::set_global_default(FmtSubscriber::default());

    let app = router::create_route();
    info!("Starting server http://0.0.0.0:8080");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
