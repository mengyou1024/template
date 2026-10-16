//! Application configuration.

use std::time::Duration;

use anyhow::Result;
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
    pub idle_timeout_secs: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:postgres@localhost:5432/{{project-name}}".to_string(),
            max_connections: 10,
            min_connections: 1,
            connect_timeout_secs: 5,
            idle_timeout_secs: 300,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AuthConfig {
    pub password_hash_cost: u32,
    pub session_timeout_secs: u64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            password_hash_cost: 12,
            session_timeout_secs: 3600,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
        }
    }
}

impl AppConfig {
    /// Load configuration from defaults, TOML file, and environment variables.
    ///
    /// Priority (highest to lowest):
    /// 1. Environment variables (prefixed with `APP_`)
    /// 2. `config.toml` file
    /// 3. Default values
    pub fn load() -> Result<Self> {
        let config: Self = Figment::from(Serialized::defaults(Self::default()))
            .merge(Toml::file("config.toml"))
            .merge(Env::prefixed("APP_").split("_"))
            .extract()?;
        Ok(config)
    }

    pub fn connect_timeout(&self) -> Duration {
        Duration::from_secs(self.database.connect_timeout_secs)
    }

    pub fn idle_timeout(&self) -> Duration {
        Duration::from_secs(self.database.idle_timeout_secs)
    }
}
