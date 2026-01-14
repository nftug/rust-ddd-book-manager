use std::sync::Arc;

use domain::author::interface::{AuthorDomainQueryService, AuthorRepository};

use crate::author::service::AuthorsFactoryService;

pub struct AuthorServiceRegistry {
    authors_factory_service: Arc<AuthorsFactoryService>,
}

impl AuthorServiceRegistry {
    pub fn new(
        repository: Arc<dyn AuthorRepository>,
        domain_query_service: Arc<dyn AuthorDomainQueryService>,
    ) -> Self {
        let authors_factory_service = AuthorsFactoryService::new(repository, domain_query_service);

        AuthorServiceRegistry {
            authors_factory_service: Arc::new(authors_factory_service),
        }
    }

    pub fn authors_factory_service(&self) -> Arc<AuthorsFactoryService> {
        self.authors_factory_service.clone()
    }
}
