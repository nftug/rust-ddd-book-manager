use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{Actor, AuditContext, Clock},
    user::{
        entity::User,
        interface::{UserDomainQueryService, UserRepository},
    },
};

use crate::{shared::error::ApplicationError, user::dto::GetOrCreateUserRequestDTO};

#[derive(new)]
pub struct GetOrCreateActorService {
    clock: Arc<dyn Clock>,
    user_domain_query_service: Arc<dyn UserDomainQueryService>,
    user_repository: Arc<dyn UserRepository>,
}

impl GetOrCreateActorService {
    pub async fn execute(
        &self,
        request: GetOrCreateUserRequestDTO,
    ) -> Result<Actor, ApplicationError> {
        if let Some(actor) = self
            .user_domain_query_service
            .find_actor_by_id(request.id.into())
            .await?
        {
            Ok(actor)
        } else {
            let context = AuditContext::new(&Actor::new_system(), self.clock.as_ref());

            let new_user = User::create_new(
                &context,
                request.id.into(),
                request.name.try_into()?,
                request.email.try_into()?,
                request.role,
            )?;

            self.user_repository.save(&new_user).await?;

            Ok(Actor::hydrate(
                new_user.audit().id().into(),
                new_user.name().into(),
                new_user.role(),
            ))
        }
    }
}
