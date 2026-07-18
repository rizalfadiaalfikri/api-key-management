use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_db(databse_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(databse_url)
        .await?;

    Ok(pool)
}