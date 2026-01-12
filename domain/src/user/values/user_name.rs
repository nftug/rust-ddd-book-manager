use crate::shared::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    pub fn hydrate(name: String) -> Self {
        Self(name)
    }

    pub fn raw(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for UserName {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value {
            n if n.is_empty() => Err(DomainError::ValidationError(
                "User name cannot be empty".to_string(),
            )),
            n if n.len() > 100 => Err(DomainError::ValidationError(
                "User name cannot exceed 100 characters".to_string(),
            )),
            n => Ok(Self(n)),
        }
    }
}
