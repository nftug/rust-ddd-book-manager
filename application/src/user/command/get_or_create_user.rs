use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::{AuditContext, Clock},
    auth::Actor,
    user::{entity::User, interface::UserRepository, values::*},
};

use crate::{shared::error::ApplicationError, user::dto::GetOrCreateUserRequestDTO};

#[derive(new)]
pub struct GetOrCreateActorService {
    clock: Arc<dyn Clock>,
    user_repository: Arc<dyn UserRepository>,
}

impl GetOrCreateActorService {
    pub async fn execute(
        &self,
        request: GetOrCreateUserRequestDTO,
    ) -> Result<Actor, ApplicationError> {
        if let Some(user) = self.user_repository.find(request.id.into()).await? {
            Ok(user.into())
        } else {
            let context = AuditContext::new(&Actor::new_system(), self.clock.as_ref());

            let new_user = User::create_new(
                &context,
                request.id.into(),
                UserName::try_new(request.name)?,
                UserEmail::try_new(request.email)?,
                request.role,
            )?;

            self.user_repository.save(&new_user).await?;

            Ok(new_user.into())
        }
    }
}
