use crate::shared::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookDescription(Option<String>);

impl BookDescription {
    pub fn hydrate(value: Option<String>) -> Self {
        Self(value)
    }

    pub fn raw(&self) -> Option<&str> {
        self.0.as_deref()
    }
}

impl TryFrom<Option<String>> for BookDescription {
    type Error = DomainError;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        match value {
            Some(ref d) if d.len() > 1000 => Err(DomainError::ValidationError(
                "Book description cannot exceed 1000 characters".to_string(),
            )),
            _ => Ok(Self(value)),
        }
    }
}
