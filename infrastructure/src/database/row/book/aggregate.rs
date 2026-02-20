use application::book::dto::{BookDetailsDTO, BookListItemDTO};
use domain::{
    auth::permission::Permission,
    book::{
        entity::Book,
        values::{BookAuthorName, BookId},
    },
};
use itertools::Itertools;

use crate::{
    database::row::{
        book::rows::{BookAuthorReferenceRow, BookDetailsRow, BookListItemRow},
        rows::BookCheckoutRow,
    },
    macros::{hydrate_audit, hydrate_audit_dto, hydrate_audit_summary_dto},
};

pub struct AggregatedBookDetails {
    pub row: BookDetailsRow,
    pub authors: Vec<BookAuthorReferenceRow>,
    pub checkouts: Vec<BookCheckoutRow>,
}

impl AggregatedBookDetails {
    pub fn from_rows(rows: Vec<BookDetailsRow>) -> Option<Self> {
        Some(Self {
            row: rows.first()?.clone(),
            authors: rows
                .iter()
                .map(|r| r.author.clone())
                .unique_by(|a| a.order_index)
                .collect(),
            checkouts: rows
                .iter()
                .filter_map(|r| r.checkout.clone())
                .unique_by(|c| c.checkout_id)
                .collect(),
        })
    }

    pub fn to_dto<T: Permission>(self, permission: T) -> BookDetailsDTO {
        BookDetailsDTO {
            id: self.row.id,
            title: self.row.title,
            authors: self
                .authors
                .into_iter()
                .unique_by(|a| a.order_index)
                .map(|a| a.name)
                .collect(),
            isbn: self.row.isbn,
            description: self.row.description,
            owner: self.row.user.to_dto(),
            checkout: self
                .checkouts
                .into_iter()
                .filter(|c| c.returned_at.is_none())
                .max_by_key(|c| c.checked_out_at)
                .map(|c| c.to_dto()),
            audit: hydrate_audit_dto!(self.row, permission),
        }
    }

    pub fn to_entity(self) -> Book {
        let authors_with_index: Vec<(BookAuthorName, usize)> = self
            .authors
            .into_iter()
            .map(|a| (a.to_domain(), a.order_index as usize))
            .collect();

        Book::hydrate(
            hydrate_audit!(self.row, BookId),
            self.row.title,
            authors_with_index,
            self.row.isbn,
            self.row.description,
            self.row.user.to_domain(),
            self.checkouts.into_iter().map(|c| c.to_domain()).collect(),
        )
    }
}

pub struct AggregatedBookListItem {
    pub row: BookListItemRow,
    pub authors: Vec<BookAuthorReferenceRow>,
    pub checkouts: Vec<BookCheckoutRow>,
}

impl AggregatedBookListItem {
    pub fn from_rows(rows: Vec<BookListItemRow>) -> Vec<Self> {
        rows.into_iter()
            .chunk_by(|r| r.id)
            .into_iter()
            .map(|(_, group)| {
                let group: Vec<BookListItemRow> = group.collect();
                AggregatedBookListItem {
                    row: group[0].clone(),
                    authors: group
                        .iter()
                        .map(|r| r.author.clone())
                        .unique_by(|a| a.order_index)
                        .collect(),
                    checkouts: group
                        .iter()
                        .filter_map(|r| r.checkout.clone())
                        .unique_by(|c| c.checkout_id)
                        .collect(),
                }
            })
            .collect()
    }

    pub fn to_dto<T: Permission>(self, permission: T) -> BookListItemDTO {
        BookListItemDTO {
            id: self.row.id,
            title: self.row.title,
            authors: self.authors.into_iter().map(|a| a.name).collect(),
            owner: self.row.user.to_dto(),
            checked_out: self.checkouts.into_iter().any(|c| c.returned_at.is_none()),
            audit: hydrate_audit_summary_dto!(self.row, permission),
        }
    }
}
