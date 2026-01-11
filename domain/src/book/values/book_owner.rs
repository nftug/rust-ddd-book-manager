use crate::{shared::error::DomainError, user::UserReference};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BookOwner(Option<UserReference>);

impl BookOwner {
    pub fn new(owner: Option<UserReference>) -> Self {
        BookOwner(owner)
    }

    pub fn hydrate(owner: Option<UserReference>) -> Self {
        BookOwner(owner)
    }

    pub fn raw(&self) -> &Option<UserReference> {
        &self.0
    }

    pub fn update(&self, new_owner: Option<UserReference>) -> Result<Self, DomainError> {
        if self.0 == new_owner {
            Err(DomainError::ValidationError(
                "Book owner is the same as the current one".to_string(),
            ))
        } else {
            Ok(BookOwner(new_owner))
        }
    }
}
