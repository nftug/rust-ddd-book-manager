use derive_new::new;

use crate::shared::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq, new)]
pub struct BookTitle(String);

impl BookTitle {
    pub fn try_new(title: String) -> Result<Self, DomainError> {
        match title {
            t if t.is_empty() => Err(DomainError::ValidationError(
                "Book title cannot be empty".to_string(),
            )),
            t if t.len() > 255 => Err(DomainError::ValidationError(
                "Book title cannot exceed 255 characters".to_string(),
            )),
            t => Ok(BookTitle(t)),
        }
    }

    pub fn hydrate(title: String) -> Self {
        BookTitle(title)
    }

    pub fn raw(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for BookTitle {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        BookTitle::try_new(value)
    }
}
