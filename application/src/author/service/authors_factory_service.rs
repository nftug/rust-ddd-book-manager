use std::sync::Arc;

use derive_new::new;
use domain::{
    audit::AuditContext,
    author::{
        entity::Author,
        interface::{AuthorDomainQueryService, AuthorRepository},
        values::{AuthorName, AuthorReference},
    },
    shared::error::DomainError,
};
use itertools::Itertools;

use crate::shared::error::ApplicationError;

#[derive(new)]
pub struct AuthorsFactoryService {
    author_repository: Arc<dyn AuthorRepository>,
    author_domain_query_service: Arc<dyn AuthorDomainQueryService>,
}

impl AuthorsFactoryService {
    pub async fn ensure_authors_exist(
        &self,
        context: &AuditContext,
        author_names: &[AuthorName],
    ) -> Result<Vec<AuthorReference>, ApplicationError> {
        let uniq_author_names: Vec<AuthorName> = author_names
            .iter()
            .unique_by(|an| an.raw())
            .cloned()
            .collect();

        if author_names.len() != uniq_author_names.len() {
            return Err(DomainError::ValidationError(
                "Author names contain duplicates".to_string(),
            )
            .into());
        }

        let mut author_refs = self
            .author_domain_query_service
            .find_author_refs_by_name(author_names)
            .await?;

        // 存在しない著者がいた場合は作成する
        for name in author_names {
            if !author_refs.iter().any(|ar| ar.name() == name) {
                let new_author = Author::create_new(context, name.clone())?;
                self.author_repository.save(&new_author).await?;
                author_refs.push(new_author.reference());
            }
        }

        Ok(author_refs)
    }
}
