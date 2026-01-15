use uuid::Uuid;

use crate::{
    author::values::{AuthorName, AuthorReference},
    shared::error::DomainError,
};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookAuthorList(Vec<OrderedAuthorReference>);

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OrderedAuthorReference(AuthorReference, usize);

impl BookAuthorList {
    pub fn hydrate(author_references: Vec<(AuthorReference, usize)>) -> Self {
        Self(
            author_references
                .into_iter()
                .map(|(x, idx)| OrderedAuthorReference(x, idx))
                .collect(),
        )
    }

    pub fn try_new(
        ordered_author_names: Vec<AuthorName>,
        author_refs: Vec<AuthorReference>,
    ) -> Result<Self, DomainError> {
        if ordered_author_names.is_empty() {
            return Err(DomainError::ValidationError(
                "Author list cannot be empty".to_string(),
            ));
        }

        let author_refs_set: Vec<_> = author_refs.iter().unique_by(|ar| ar.raw_id()).collect();
        if ordered_author_names.len() > author_refs_set.len() {
            return Err(DomainError::ValidationError(
                "Author list contains duplicate authors".to_string(),
            ));
        }
        if ordered_author_names.len() < author_refs_set.len() {
            return Err(DomainError::ValidationError(
                "Some authors do not exist".to_string(),
            ));
        }

        // 入力値であるordered_raw_namesの順番を保持する
        Ok(Self(
            ordered_author_names
                .into_iter()
                .enumerate()
                .filter_map(|(idx, name)| {
                    author_refs_set
                        .iter()
                        .find(|author_ref| author_ref.name() == &name)
                        .map(|&author_ref| OrderedAuthorReference(author_ref.clone(), idx))
                })
                .collect(),
        ))
    }

    pub fn raw(&self) -> &[OrderedAuthorReference] {
        &self.0
    }

    pub fn raw_ids(&self) -> Vec<Uuid> {
        self.raw()
            .iter()
            .map(|ordered_author| ordered_author.0.raw_id())
            .collect()
    }
}

impl OrderedAuthorReference {
    pub fn raw_id(&self) -> Uuid {
        self.0.raw_id()
    }

    pub fn order_index(&self) -> usize {
        self.1
    }
}
