use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BackendSettings {
    pub engine: String,
    pub use_trait_objects: Option<bool>, // Optional: Defaults to false if missing
}

#[derive(Debug, Deserialize)]
pub struct ActixSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct WarpSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub engine: String,
}

#[derive(Debug, Deserialize)]
pub struct FrontendSettings {
    pub engine: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub backend: BackendSettings,
    pub actix: Option<ActixSettings>, // Optional: Only used when Actix is selected
    pub warp: Option<WarpSettings>,   // Optional: Only used when Warp is selected
    pub database: DatabaseSettings,
    pub frontend: FrontendSettings,
}

pub fn load_config() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("config.toml"))
        .set_default("backend.engine", "actix")?
        .set_default("backend.use_trait_objects", false)? // Default to struct-based
        .set_default("actix.host", "127.0.0.1")?
        .set_default("actix.port", 8080)?
        .set_default("warp.host", "127.0.0.1")?
        .set_default("warp.port", 8081)?
        .set_default("database.engine", "postgres")?
        .set_default("frontend.engine", "yew")?
        .build()?;

    settings.try_deserialize()
}
