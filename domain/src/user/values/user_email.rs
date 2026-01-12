use email_address::EmailAddress;

use crate::shared::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn hydrate(email: String) -> Self {
        Self(email)
    }

    pub fn raw(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for UserEmail {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if EmailAddress::is_valid(&value) {
            Ok(Self(value))
        } else {
            Err(DomainError::ValidationError(
                "Invalid email address format".to_string(),
            ))
        }
    }
}
