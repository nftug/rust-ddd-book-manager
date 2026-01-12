use std::env;

use strum::EnumString;

pub struct AppConfig {
    pub environment: Environment,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub oidc: OidcConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(AppConfig {
            environment: Environment::new(),
            server: ServerConfig::new()?,
            database: DatabaseConfig::new()?,
            oidc: OidcConfig::new()?,
        })
    }
}

#[derive(Default, EnumString, Debug)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    #[default]
    Development,
    Production,
}

impl Environment {
    pub fn new() -> Self {
        #[cfg(debug_assertions)]
        let default_env = Environment::Development;
        #[cfg(not(debug_assertions))]
        let default_env = Environment::Production;

        env::var("APP_ENV")
            .ok()
            .and_then(|env_str| env_str.parse().ok())
            .unwrap_or(default_env)
    }
}

pub struct ServerConfig {
    pub port: u16,
}

impl ServerConfig {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ServerConfig {
            port: env::var("PORT")?.parse()?,
        })
    }
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(DatabaseConfig {
            host: env::var("DATABASE_HOST")?,
            port: env::var("DATABASE_PORT")?.parse()?,
            username: env::var("DATABASE_USERNAME")?,
            password: env::var("DATABASE_PASSWORD")?,
            database: env::var("DATABASE_NAME")?,
        })
    }
}

pub struct OidcConfig {
    pub authority: String,
    pub client_id: String,
    pub audience: Option<String>,
}

impl OidcConfig {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(OidcConfig {
            authority: env::var("OIDC_AUTHORITY")?,
            client_id: env::var("OIDC_CLIENT_ID")?,
            audience: env::var("OIDC_AUDIENCE").ok(),
        })
    }
}
