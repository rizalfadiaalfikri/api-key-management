use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{app::create_app, db::init_db, settings::Settings, state::AppState};

pub mod app;
pub mod state;
pub mod db;
pub mod errors;
pub mod settings;
pub mod utils;
pub mod routes;



#[tokio::main]
async  fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let settings = Settings::new()?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(settings.rust_log.clone()))
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    let db_pool = init_db(&settings.database_url).await?;
    let state = AppState { db: db_pool };
    let app = create_app(state);

    let addr = format!("{}:{}", settings.app_host, settings.app_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server running on http://{}", addr);
    tracing::info!("Swagger docs available on http://{}/swagger-ui", addr);

    axum::serve(listener, app).await?;
    Ok(())
    
}