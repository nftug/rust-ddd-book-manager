use email_address::EmailAddress;

use crate::shared::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn new(email: String) -> Result<Self, DomainError> {
        if EmailAddress::is_valid(&email) {
            Ok(UserEmail(email))
        } else {
            Err(DomainError::ValidationError(
                "Invalid email address format".to_string(),
            ))
        }
    }

    pub fn hydrate(email: String) -> Self {
        UserEmail(email)
    }

    pub fn raw(&self) -> &str {
        &self.0
    }
}
