use std::str::FromStr;

use application::{shared::UserReferenceDTO, user::dto::UserDetailsDTO};
use domain::{
    auth::Actor,
    shared::error::PersistenceError,
    user::{enums::UserRole, values::UserReference},
};
use sea_orm::FromQueryResult;
use uuid::Uuid;

#[derive(Debug, FromQueryResult)]
pub struct UserReferenceRow {
    pub id: Uuid,
    pub name: String,
}

impl From<UserReferenceRow> for UserReference {
    fn from(row: UserReferenceRow) -> Self {
        UserReference::hydrate(row.id, row.name)
    }
}

impl From<UserReferenceRow> for UserReferenceDTO {
    fn from(row: UserReferenceRow) -> Self {
        UserReferenceDTO {
            id: row.id,
            name: row.name,
        }
    }
}

#[derive(Debug, FromQueryResult)]
pub struct UserDetailsRow {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
}

impl UserDetailsRow {
    pub fn to_dto(self) -> Result<UserDetailsDTO, PersistenceError> {
        Ok(UserDetailsDTO {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            role: UserRole::from_str(&self.role)
                .map_err(|e| PersistenceError::EntityConversionError(e.to_string()))?,
        })
    }
}

#[derive(Debug, FromQueryResult)]
pub struct ActorRow {
    pub id: Uuid,
    pub name: String,
    pub role: String,
}

impl ActorRow {
    pub fn to_actor(self) -> Result<Actor, PersistenceError> {
        Ok(Actor::hydrate(
            self.id,
            self.name,
            UserRole::from_str(&self.role)
                .map_err(|e| PersistenceError::EntityConversionError(e.to_string()))?,
        ))
    }
}
