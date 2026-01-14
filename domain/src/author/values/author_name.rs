use crate::shared::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorName(String);

impl AuthorName {
    pub fn hydrate(name: String) -> Self {
        Self(name)
    }

    pub fn raw(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for AuthorName {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value {
            n if n.is_empty() => Err(DomainError::ValidationError(
                "Author name cannot be empty".to_string(),
            )),
            n if n.len() > 255 => Err(DomainError::ValidationError(
                "Author name cannot exceed 255 characters".to_string(),
            )),
            n => Ok(Self(n)),
        }
    }
}
