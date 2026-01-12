use std::sync::Arc;

use domain::{audit::Clock, user::interface::UserRepository};

use crate::user::{command::*, dto::*};

pub struct UserRegistry {
    get_or_create_user: Arc<GetOrCreateActorService>,
}

impl UserRegistry {
    pub fn new(user_repository: Arc<dyn UserRepository>, clock: Arc<dyn Clock>) -> Self {
        let get_or_create_user =
            GetOrCreateActorService::new(clock.clone(), user_repository.clone());

        UserRegistry {
            get_or_create_user: Arc::new(get_or_create_user),
        }
    }

    pub async fn get_or_create_user(
        &self,
        request: GetOrCreateUserRequestDTO,
    ) -> Result<domain::auth::Actor, crate::shared::error::ApplicationError> {
        self.get_or_create_user.execute(request).await
    }
}
