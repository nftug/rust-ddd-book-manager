use async_trait::async_trait;
use derive_new::new;
use domain::{
    author::{entity::Author, interface::AuthorRepository, values::*},
    shared::error::PersistenceError,
};
use sea_orm::{ActiveValue::Set, EntityTrait};

use crate::{
    database::{ConnectionPool, entity::authors, log_db_error},
    macros::{audit_defaults, hydrate_audit, update_on_conflict},
};

#[derive(new)]
pub struct AuthorRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl AuthorRepository for AuthorRepositoryImpl {
    async fn find_by_id(&self, id: AuthorId) -> Result<Option<Author>, PersistenceError> {
        let result = authors::Entity::find_by_id(id)
            .one(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        Ok(result.map(|model| {
            let audit = hydrate_audit!(model, AuthorId);
            Author::hydrate(audit, model.name)
        }))
    }

    async fn save(&self, author: &Author) -> Result<(), PersistenceError> {
        let active_model = authors::ActiveModel {
            id: Set(author.audit().raw_id()),
            name: Set(author.name().to_string()),
            ..audit_defaults!(authors::ActiveModel, author.audit())
        };

        authors::Entity::insert(active_model)
            .on_conflict(update_on_conflict!(authors::Column))
            .exec(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        Ok(())
    }

    async fn delete(&self, id: AuthorId) -> Result<(), PersistenceError> {
        let result = authors::Entity::delete_by_id(id)
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
