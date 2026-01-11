use derive_new::new;

use crate::{
    shared::error::DomainError,
    user::values::{UserId, UserReference},
};

#[derive(Debug, Clone, PartialEq, Eq, new)]
pub struct BookOwner(UserReference);

impl BookOwner {
    pub fn id(&self) -> UserId {
        self.0.id()
    }
    pub fn name(&self) -> &str {
        self.0.name()
    }
    pub fn raw(&self) -> &UserReference {
        &self.0
    }

    pub fn update(&self, new_owner: UserReference) -> Result<Self, DomainError> {
        if self.0 == new_owner {
            Err(DomainError::ValidationError(
                "Book owner is the same as the current one".to_string(),
            ))
        } else {
            Ok(BookOwner(new_owner))
        }
    }
}
