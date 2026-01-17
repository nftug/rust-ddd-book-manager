use std::sync::Arc;

use domain::{
    audit::Clock,
    user::interface::{UserDomainQueryService, UserRepository},
};

use crate::user::{command::*, interface::*, query::*};

pub struct UserRegistry {
    get_or_create_user: Arc<GetOrCreateActorService>,
    get_user_details: Arc<GetUserDetailsService>,
}

impl UserRegistry {
    pub fn new(
        repository: Arc<dyn UserRepository>,
        query_service: Arc<dyn UserQueryService>,
        domain_query_service: Arc<dyn UserDomainQueryService>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        let get_or_create_actor = GetOrCreateActorService::new(
            clock.clone(),
            domain_query_service.clone(),
            repository.clone(),
        );
        let get_user_details = GetUserDetailsService::new(query_service.clone());

        UserRegistry {
            get_or_create_user: Arc::new(get_or_create_actor),
            get_user_details: Arc::new(get_user_details),
        }
    }

    pub fn get_or_create_user(&self) -> Arc<GetOrCreateActorService> {
        self.get_or_create_user.clone()
    }

    pub fn get_user_details(&self) -> Arc<GetUserDetailsService> {
        self.get_user_details.clone()
    }
}
