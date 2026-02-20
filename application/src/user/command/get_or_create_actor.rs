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
        request: &GetOrCreateUserRequestDTO,
    ) -> Result<Actor, ApplicationError> {
        let context = AuditContext::new(&Actor::new_system(), self.clock.as_ref());

        if let Some(actor) = self
            .user_domain_query_service
            .find_actor_by_id(request.id)
            .await?
        {
            // If the user info from the request is different from the existing user, update it
            if actor.name() != request.name || actor.role() != request.role.into() {
                let mut user_from_request = self
                    .user_repository
                    .find_by_id(request.id)
                    .await?
                    .ok_or(ApplicationError::InternalError("User not found".into()))?;

                user_from_request.update(
                    &context,
                    request.name.clone().try_into()?,
                    request.email.clone().try_into()?,
                    request.role.into(),
                )?;

                self.user_repository.save(&user_from_request).await?;

                Ok(user_from_request.into_actor())
            } else {
                Ok(actor)
            }
        } else {
            let new_user = User::create_new(
                &context,
                request.id,
                request.name.clone().try_into()?,
                request.email.clone().try_into()?,
                request.role.into(),
            )?;

            self.user_repository.save(&new_user).await?;

            Ok(new_user.into_actor())
        }
    }
}
