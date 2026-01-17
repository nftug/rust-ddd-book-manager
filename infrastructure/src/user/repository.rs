use std::str::FromStr;

use async_trait::async_trait;
use derive_new::new;
use domain::{
    shared::error::PersistenceError,
    user::{entity::User, enums::UserRole, interface::UserRepository, values::*},
};
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};

use crate::{
    database::{ConnectionPool, entity::users, log_db_error},
    macros::{audit_defaults, hydrate_audit},
};

#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, PersistenceError> {
        let result = users::Entity::find_by_id(id)
            .one(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        match result {
            Some(user) => {
                let audit = hydrate_audit!(user, UserId);
                Ok(Some(User::hydrate(
                    audit,
                    user.name,
                    user.email,
                    UserRole::from_str(&user.role)
                        .map_err(|e| PersistenceError::EntityConversionError(e.to_string()))?,
                )))
            }
            None => Ok(None),
        }
    }

    async fn save(&self, user: &User) -> Result<(), PersistenceError> {
        let active_model = users::ActiveModel {
            name: Set(user.name().into()),
            email: Set(user.email().into()),
            role: Set(user.role().as_ref().into()),
            ..audit_defaults!(users::ActiveModel, user.audit())
        };

        let exists = users::Entity::find()
            .filter(users::Column::Id.eq(user.audit().raw_id()))
            .count(self.db.inner_ref())
            .await
            .map_err(log_db_error)?
            > 0;

        if exists {
            users::Entity::update(active_model)
                .exec(self.db.inner_ref())
                .await
                .map_err(log_db_error)?;
        } else {
            users::Entity::insert(active_model)
                .exec(self.db.inner_ref())
                .await
                .map_err(log_db_error)?;
        }

        Ok(())
    }

    async fn delete(&self, id: UserId) -> Result<(), PersistenceError> {
        let result = users::Entity::delete_by_id(id)
            .exec(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        if result.rows_affected == 0 {
            Err(PersistenceError::NotFound)
        } else {
            Ok(())
        }
    }
}
