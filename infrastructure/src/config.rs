pub struct AppConfig {
    pub database: DatabaseConfig,
    pub oidc: OidcConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let database = DatabaseConfig {
            host: std::env::var("DATABASE_HOST")?,
            port: std::env::var("DATABASE_PORT")?.parse()?,
            username: std::env::var("DATABASE_USERNAME")?,
            password: std::env::var("DATABASE_PASSWORD")?,
            database: std::env::var("DATABASE_NAME")?,
        };

        let oidc = OidcConfig {
            authority: std::env::var("OIDC_AUTHORITY")?,
            client_id: std::env::var("OIDC_CLIENT_ID")?,
        };

        Ok(AppConfig { database, oidc })
    }
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

pub struct OidcConfig {
    pub authority: String,
    pub client_id: String,
}
