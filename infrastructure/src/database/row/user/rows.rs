use std::str::FromStr;

use application::{
    shared::UserReferenceDTO,
    user::dto::{UserDetailsDTO, UserRoleDTO},
};
use domain::{
    audit::Actor,
    shared::error::PersistenceError,
    user::{enums::UserRole, values::UserReference},
};
use sea_orm::DerivePartialModel;
use uuid::Uuid;

#[derive(DerivePartialModel, Clone)]
#[sea_orm(entity = "crate::database::entity::users::Entity")]
pub struct UserReferenceRow {
    pub id: Uuid,
    pub name: String,
}

impl UserReferenceRow {
    pub fn to_domain(self) -> UserReference {
        UserReference::hydrate(self.id, self.name)
    }

    pub fn to_dto(self) -> UserReferenceDTO {
        UserReferenceDTO {
            id: self.id,
            name: self.name,
        }
    }
}

#[derive(DerivePartialModel)]
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
            name: self.name,
            email: self.email,
            role: UserRoleDTO::from_str(&self.role)
                .map_err(|e| PersistenceError::EntityConversionError(e.to_string()))?,
        })
    }
}

#[derive(DerivePartialModel)]
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
