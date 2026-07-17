use std::net::SocketAddr;
use tokio::net::TcpListener;

use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::{config::Config, state::AppState};

pub mod config;
pub mod proxy;
pub mod router;
pub mod state;


#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();
    init_tracing();

    let config = Config::from_env().expect("gagal memuat konfigurasi environment");
    let state = AppState::new(config.clone()).expect("gagal membuat app state");

    let app = router::create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("API Gateway listening at http://{}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .expect("gagal bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("gagal menjalankan server");

}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(true)
        .compact()
        .init();
}