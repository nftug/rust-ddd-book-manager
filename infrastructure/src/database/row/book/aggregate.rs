use application::book::dto::{BookDetailsDTO, BookListItemDTO};
use domain::{
    auth::permission::Permission,
    author::values::AuthorReference,
    book::{entity::Book, values::BookId},
};
use itertools::Itertools;

use crate::{
    database::row::{
        author::BookAuthorRow,
        book::rows::{BookDetailsRow, BookListItemRow},
    },
    macros::{hydrate_audit, hydrate_audit_dto, hydrate_audit_summary_dto},
};

pub struct AggregatedBookDetails {
    pub row: BookDetailsRow,
    pub authors: Vec<BookAuthorRow>,
}

impl AggregatedBookDetails {
    pub fn from_rows(rows: Vec<BookDetailsRow>) -> Option<Self> {
        Some(Self {
            row: rows.first()?.clone(),
            authors: rows.into_iter().map(|r| r.author).collect(),
        })
    }

    pub fn to_dto(self, permission: &dyn Permission) -> BookDetailsDTO {
        BookDetailsDTO {
            id: self.row.id,
            title: self.row.title,
            authors: self.authors.into_iter().map(|a| a.to_dto()).collect(),
            isbn: self.row.isbn,
            description: self.row.description,
            owner: self.row.user.to_dto(),
            audit: hydrate_audit_dto!(self.row, permission),
        }
    }

    pub fn to_entity(self) -> Book {
        let authors_with_index: Vec<(AuthorReference, usize)> = self
            .authors
            .into_iter()
            .enumerate()
            .map(|(idx, a)| (a.to_domain(), idx))
            .collect();

        Book::hydrate(
            hydrate_audit!(self.row, BookId),
            self.row.title,
            authors_with_index,
            self.row.isbn,
            self.row.description,
            self.row.user.to_domain(),
        )
    }
}

pub struct AggregatedBookListItem {
    pub row: BookListItemRow,
    pub authors: Vec<BookAuthorRow>,
}

impl AggregatedBookListItem {
    pub fn from_rows(rows: Vec<BookListItemRow>) -> Vec<Self> {
        rows.into_iter()
            .into_group_map_by(|r| r.id)
            .into_values()
            .map(|group| AggregatedBookListItem {
                row: group[0].clone(),
                authors: group.into_iter().map(|r| r.author).collect(),
            })
            .collect()
    }

    pub fn to_dto(self, permission: &dyn Permission) -> BookListItemDTO {
        BookListItemDTO {
            id: self.row.id,
            title: self.row.title,
            authors: self.authors.into_iter().map(|a| a.to_dto()).collect(),
            owner: self.row.user.to_dto(),
            audit: hydrate_audit_summary_dto!(self.row, permission),
        }
    }
}
