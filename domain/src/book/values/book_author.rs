use crate::shared::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookAuthor(String);

impl BookAuthor {
    pub fn try_new(author: String) -> Result<Self, DomainError> {
        match author {
            a if a.is_empty() => Err(DomainError::ValidationError(
                "Book author cannot be empty".to_string(),
            )),
            a if a.len() > 255 => Err(DomainError::ValidationError(
                "Book author cannot exceed 255 characters".to_string(),
            )),
            a => Ok(BookAuthor(a)),
        }
    }

    pub fn hydrate(author: String) -> Self {
        BookAuthor(author)
    }

    pub fn raw(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for BookAuthor {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        BookAuthor::try_new(value)
    }
}
