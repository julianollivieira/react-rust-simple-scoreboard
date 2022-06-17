use serde::Deserialize;

// Config struct to hold environment variables read from the .env file.
#[derive(Deserialize)]
pub struct EnvConfig {
    pub api_port: u16,
    pub database_url: String,
}
