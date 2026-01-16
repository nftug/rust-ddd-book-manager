use std::sync::Arc;

use application::{
    author::registry::AuthorServiceRegistry, book::BookRegistry, user::UserRegistry,
};
use domain::audit::{Actor, Clock, clock::SystemClock};
use infrastructure::{
    author::{AuthorDomainQueryServiceImpl, AuthorRepositoryImpl},
    book::{BookQueryServiceImpl, BookRepositoryImpl},
    config::AppConfig,
    database::ConnectionPool,
    user::{UserDomainQueryServiceImpl, UserQueryServiceImpl, UserRepositoryImpl},
};

use crate::{auth::OidcUserInfo, error::ApiError};

#[derive(Clone)]
pub struct AppRegistry {
    config: Arc<AppConfig>,
    book_registry: Arc<BookRegistry>,
    user_registry: Arc<UserRegistry>,
}

impl AppRegistry {
    pub async fn build(
        config: AppConfig,
        clock: impl Clock + 'static,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config = Arc::new(config);
        let clock = Arc::new(clock);
        let db = ConnectionPool::new(&config.database).await?;

        let book_repository = Arc::new(BookRepositoryImpl::new(db.clone()));
        let book_query_service = Arc::new(BookQueryServiceImpl::new(db.clone()));

        let user_repository = Arc::new(UserRepositoryImpl::new(db.clone()));
        let user_query_service = Arc::new(UserQueryServiceImpl::new(db.clone()));
        let user_domain_query_service = Arc::new(UserDomainQueryServiceImpl::new(db.clone()));

        let author_repository = Arc::new(AuthorRepositoryImpl::new(db.clone()));
        let author_domain_query_service = Arc::new(AuthorDomainQueryServiceImpl::new(db.clone()));
        let author_service_registry = Arc::new(AuthorServiceRegistry::new(
            author_repository,
            author_domain_query_service,
        ));

        let book_registry = BookRegistry::new(
            book_repository,
            book_query_service,
            author_service_registry.authors_factory_service(),
            clock.clone(),
        );
        let user_registry = UserRegistry::new(
            user_repository,
            user_query_service,
            user_domain_query_service,
            clock.clone(),
        );

        Ok(AppRegistry {
            config,
            book_registry: Arc::new(book_registry),
            user_registry: Arc::new(user_registry),
        })
    }

    pub async fn build_runtime() -> Result<Self, Box<dyn std::error::Error>> {
        let config = AppConfig::from_env()?;
        let clock = SystemClock {};
        Self::build(config, clock).await
    }

    pub async fn prepare_actor(&self, user_info: OidcUserInfo) -> Result<Actor, ApiError> {
        self.user_registry()
            .get_or_create_actor(user_info.try_into()?)
            .await
            .map_err(|e| e.into())
    }

    pub async fn prepare_optional_actor(
        &self,
        user_info: Option<OidcUserInfo>,
    ) -> Result<Option<Actor>, ApiError> {
        if let Some(info) = user_info {
            let actor = self
                .user_registry()
                .get_or_create_actor(info.try_into()?)
                .await?;
            Ok(Some(actor))
        } else {
            Ok(None)
        }
    }

    pub fn config(&self) -> Arc<AppConfig> {
        Arc::clone(&self.config)
    }

    pub fn book_registry(&self) -> Arc<BookRegistry> {
        Arc::clone(&self.book_registry)
    }

    pub fn user_registry(&self) -> Arc<UserRegistry> {
        Arc::clone(&self.user_registry)
    }
}
