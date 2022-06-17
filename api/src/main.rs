mod env;
mod http;
mod models;

use anyhow::{Context, Result};
use env::EnvConfig;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse environment variables from the .env file into a struct.
    dotenv::dotenv().context("Failed to read .env file")?;
    let config = envy::from_env::<EnvConfig>().context("Failed to parse environment variables")?;

    // Create a connection pool for the database.
    let pool = PgPoolOptions::new()
        .connect(&config.database_url)
        .await
        .context("Failed to connect to the database")?;

    // Spin up the HTTP server.
    http::serve(config, pool).await?;

    Ok(())
}
