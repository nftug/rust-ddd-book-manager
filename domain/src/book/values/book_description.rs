use crate::shared::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookDescription(Option<String>);

impl BookDescription {
    pub fn try_new(description: Option<String>) -> Result<Self, DomainError> {
        match description {
            Some(ref d) if d.len() > 1000 => Err(DomainError::ValidationError(
                "Book description cannot exceed 1000 characters".to_string(),
            )),
            _ => Ok(BookDescription(description)),
        }
    }

    pub fn hydrate(description: Option<String>) -> Self {
        BookDescription(description)
    }

    pub fn raw(&self) -> Option<&str> {
        self.0.as_deref()
    }
}

impl TryFrom<Option<String>> for BookDescription {
    type Error = DomainError;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        BookDescription::try_new(value)
    }
}
