use uuid::Uuid;

use crate::{
    author::values::{AuthorId, AuthorName},
    shared::EntityIdTrait,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorReference(AuthorId, AuthorName);

impl AuthorReference {
    pub fn hydrate(author_id: Uuid, author_name: String) -> Self {
        Self(author_id.into(), AuthorName::hydrate(author_name))
    }

    pub fn from_values(author_id: AuthorId, author_name: AuthorName) -> Self {
        Self(author_id, author_name)
    }

    pub fn id(&self) -> &AuthorId {
        &self.0
    }
    pub fn name(&self) -> &AuthorName {
        &self.1
    }
    pub fn raw_id(&self) -> Uuid {
        self.0.raw()
    }
    pub fn raw_name(&self) -> &str {
        self.1.raw()
    }
}

impl From<&AuthorReference> for AuthorId {
    fn from(value: &AuthorReference) -> Self {
        value.0
    }
}

impl From<&AuthorReference> for AuthorName {
    fn from(value: &AuthorReference) -> Self {
        value.1.clone()
    }
}
