use async_trait::async_trait;
use derive_new::new;
use domain::{
    author::{
        interface::AuthorDomainQueryService,
        values::{AuthorName, AuthorReference},
    },
    shared::error::PersistenceError,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::database::{ConnectionPool, entity::authors, log_db_error, row::AuthorReferenceRow};

#[derive(new)]
pub struct AuthorDomainQueryServiceImpl {
    db: ConnectionPool,
}

#[async_trait]
impl AuthorDomainQueryService for AuthorDomainQueryServiceImpl {
    async fn find_author_refs_by_name(
        &self,
        names: &[AuthorName],
    ) -> Result<Vec<AuthorReference>, PersistenceError> {
        if names.is_empty() {
            return Ok(Vec::new());
        }

        let raw_names: Vec<String> = names.iter().map(|n| n.raw().to_string()).collect();

        let models = authors::Entity::find()
            .filter(authors::Column::Name.is_in(raw_names))
            .into_partial_model::<AuthorReferenceRow>()
            .all(self.db.inner_ref())
            .await
            .map_err(log_db_error)?;

        Ok(models.into_iter().map(|m| m.to_domain()).collect())
    }
}
