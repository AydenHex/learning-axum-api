use config::{Config, ConfigError, Environment};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database_host: String,
    pub database_user: String,
    pub database_name: String,
    pub database_password: String,
    pub database_port: i16,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();
        let settings: Config = Config::builder()
            .add_source(Environment::default())
            .build()?;

        settings.try_deserialize()
    }
}
