use crate::{book::values::BookAuthorName, shared::error::DomainError};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookAuthorList(Vec<OrderedAuthorReference>);

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct OrderedAuthorReference(BookAuthorName, usize);

impl BookAuthorList {
    pub fn hydrate(author_references: Vec<(BookAuthorName, usize)>) -> Self {
        Self(
            author_references
                .into_iter()
                .map(|(x, idx)| OrderedAuthorReference(x, idx))
                .collect(),
        )
    }

    pub fn raw(&self) -> &[OrderedAuthorReference] {
        &self.0
    }
}

impl TryFrom<Vec<String>> for BookAuthorList {
    type Error = DomainError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let names: Vec<BookAuthorName> = value.into_iter().map(|n| n.try_into()).try_collect()?;

        if names.is_empty() {
            return Err(DomainError::ValidationError(
                "Author list cannot be empty".to_string(),
            ));
        }

        let unique_names: Vec<&BookAuthorName> = names.iter().unique().collect();
        if unique_names.len() != names.len() {
            return Err(DomainError::ValidationError(
                "Author list contains duplicate names".to_string(),
            ));
        }

        Ok(Self(
            names
                .into_iter()
                .enumerate()
                .map(|(idx, name)| OrderedAuthorReference(name, idx))
                .collect(),
        ))
    }
}

impl OrderedAuthorReference {
    pub fn name(&self) -> &BookAuthorName {
        &self.0
    }

    pub fn order_index(&self) -> usize {
        self.1
    }
}
