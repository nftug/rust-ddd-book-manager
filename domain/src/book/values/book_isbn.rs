use crate::shared::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookIsbn(Option<String>);

impl BookIsbn {
    pub fn new(isbn: Option<String>) -> Result<Self, DomainError> {
        match isbn {
            Some(ref i) if i.len() != 13 => Err(DomainError::ValidationError(
                "Book ISBN must be 13 characters long".to_string(),
            )),
            _ => Ok(BookIsbn(isbn)),
        }
    }

    pub fn hydrate(isbn: Option<String>) -> Self {
        BookIsbn(isbn)
    }

    pub fn raw(&self) -> &Option<String> {
        &self.0
    }
}

impl TryFrom<Option<String>> for BookIsbn {
    type Error = DomainError;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        BookIsbn::new(value)
    }
}
