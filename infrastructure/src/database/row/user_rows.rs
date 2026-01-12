use std::str::FromStr;

use application::{shared::UserReferenceDTO, user::dto::UserDetailsDTO};
use domain::{
    auth::Actor,
    shared::error::PersistenceError,
    user::{enums::UserRole, values::UserReference},
};
use sea_orm::{DerivePartialModel, FromQueryResult};
use uuid::Uuid;

#[derive(DerivePartialModel, FromQueryResult, Clone)]
#[sea_orm(entity = "crate::database::entity::users::Entity")]
pub struct UserReferenceRow {
    #[sea_orm(from_alias = "user_id")]
    pub id: Uuid,
    #[sea_orm(from_alias = "user_name")]
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

#[derive(DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "crate::database::entity::users::Entity")]
pub struct UserDetailsDTORow {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
}

impl UserDetailsDTORow {
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

#[derive(DerivePartialModel, FromQueryResult)]
#[sea_orm(entity = "crate::database::entity::users::Entity")]
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
