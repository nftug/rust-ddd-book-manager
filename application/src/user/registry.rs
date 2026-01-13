use std::sync::Arc;

use domain::{
    audit::{Actor, Clock},
    user::interface::{UserDomainQueryService, UserRepository},
};
use uuid::Uuid;

use crate::{
    shared::error::ApplicationError,
    user::{command::*, dto::*, interface::UserQueryService, query::*},
};

pub struct UserRegistry {
    get_or_create_user: Arc<GetOrCreateActorService>,
    get_user_details: Arc<GetUserDetailsService>,
}

impl UserRegistry {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        user_query_service: Arc<dyn UserQueryService>,
        user_domain_query_service: Arc<dyn UserDomainQueryService>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        let get_or_create_actor = GetOrCreateActorService::new(
            clock.clone(),
            user_domain_query_service.clone(),
            user_repository.clone(),
        );
        let get_user_details = GetUserDetailsService::new(user_query_service.clone());

        UserRegistry {
            get_or_create_user: Arc::new(get_or_create_actor),
            get_user_details: Arc::new(get_user_details),
        }
    }

    pub async fn get_or_create_actor(
        &self,
        request: GetOrCreateUserRequestDTO,
    ) -> Result<Actor, ApplicationError> {
        self.get_or_create_user.execute(request).await
    }

    pub async fn get_user_details(
        &self,
        user_id: Uuid,
    ) -> Result<UserDetailsDTO, ApplicationError> {
        self.get_user_details.execute(user_id).await
    }
}
