use std::str::FromStr;

use async_trait::async_trait;
use derive_new::new;
use domain::{
    shared::{Id, error::PersistenceError},
    user::{entity::User, enums::UserRole, interface::UserRepository, values::*},
};
use sea_orm::{ActiveValue::Set, EntityTrait};

use crate::{
    database::{ConnectionPool, entity::users},
    macros::{audit_defaults, audit_defaults_update, hydrate_audit},
};

#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find(&self, id: UserId) -> Result<Option<User>, PersistenceError> {
        let result = users::Entity::find_by_id(id.raw())
            .one(self.db.inner_ref())
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        match result {
            Some(user) => {
                let audit = hydrate_audit!(user, UserId);
                Ok(Some(User::new(
                    audit,
                    UserName::new(user.name),
                    UserEmail::new(user.email),
                    UserRole::from_str(&user.role)
                        .map_err(|e| PersistenceError::EntityConversionError(e.to_string()))?,
                )))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, user: &User) -> Result<(), PersistenceError> {
        if user.audit().is_new() {
            let active_model = users::ActiveModel {
                name: Set(user.name().into()),
                email: Set(user.email().into()),
                role: Set(user.role().as_ref().into()),
                ..audit_defaults!(users::ActiveModel, user.audit())
            };

            users::Entity::insert(active_model)
                .exec(self.db.inner_ref())
                .await
                .map_err(|_| PersistenceError::OperationError)?;
        } else {
            let mut active_model: users::ActiveModel =
                users::Entity::find_by_id(user.audit().id().raw())
                    .one(self.db.inner_ref())
                    .await
                    .map_err(|_| PersistenceError::OperationError)?
                    .ok_or(PersistenceError::NotFound)?
                    .into();

            active_model.name = Set(user.name().into());
            active_model.email = Set(user.email().into());
            active_model.role = Set(user.role().as_ref().into());
            audit_defaults_update!(active_model, user.audit());

            users::Entity::update(active_model)
                .exec(self.db.inner_ref())
                .await
                .map_err(|_| PersistenceError::OperationError)?;
        }

        Ok(())
    }

    async fn delete(&self, id: UserId) -> Result<(), PersistenceError> {
        let result = users::Entity::delete_by_id(id.raw())
            .exec(self.db.inner_ref())
            .await
            .map_err(|_| PersistenceError::OperationError)?;

        if result.rows_affected == 0 {
            Err(PersistenceError::NotFound)
        } else {
            Ok(())
        }
    }
}
