use domain::shared::error::PersistenceError;
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, DbErr, TransactionTrait,
};

use crate::config::DatabaseConfig;

pub mod entity;
pub mod row;

impl From<&DatabaseConfig> for ConnectOptions {
    fn from(config: &DatabaseConfig) -> Self {
        let database_url = format!(
            "postgresql://{}:{}@{}:{}/{}",
            config.username, config.password, config.host, config.port, config.database
        );
        let mut options = ConnectOptions::new(database_url);
        options
            .max_connections(100)
            .min_connections(5)
            .sqlx_logging(false);
        options
    }
}

#[derive(Clone)]
pub struct ConnectionPool(DatabaseConnection);

impl ConnectionPool {
    pub fn inner_ref(&self) -> &DatabaseConnection {
        &self.0
    }

    pub async fn new(config: &DatabaseConfig) -> Result<Self, sea_orm::DbErr> {
        let options: ConnectOptions = config.into();
        Ok(Self(Database::connect(options).await?))
    }

    pub async fn begin_transaction(&self) -> Result<DatabaseTransaction, sea_orm::DbErr> {
        self.0.begin().await
    }
}

pub fn log_db_error(err: DbErr) -> PersistenceError {
    tracing::error!(error = ?err, "Database operation failed");
    PersistenceError::OperationError
}
