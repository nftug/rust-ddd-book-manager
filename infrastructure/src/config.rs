pub struct AppConfig {
    pub database: DatabaseConfig,
    pub oidc: OidcConfig,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let database = DatabaseConfig {
            host: std::env::var("DATABASE_HOST").expect("DATABASE_HOST must be set"),
            port: std::env::var("DATABASE_PORT")
                .expect("DATABASE_PORT must be set")
                .parse()
                .expect("DATABASE_PORT must be a valid u16"),
            username: std::env::var("DATABASE_USERNAME").expect("DATABASE_USERNAME must be set"),
            password: std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set"),
            database: std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"),
        };

        let oidc = OidcConfig {
            authority: std::env::var("OIDC_AUTHORITY").expect("OIDC_AUTHORITY must be set"),
            client_id: std::env::var("OIDC_CLIENT_ID").expect("OIDC_CLIENT_ID must be set"),
        };

        AppConfig { database, oidc }
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
