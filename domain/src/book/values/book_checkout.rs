use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{audit::AuditContext, shared::error::DomainError, user::values::UserReference};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookCheckoutList(Vec<BookCheckout>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookCheckout {
    Active(CheckoutRecord),
    Returned {
        checkout: CheckoutRecord,
        returned_at: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckoutRecord {
    checkout_id: Uuid,
    checked_out_to: UserReference,
    checked_out_at: DateTime<Utc>,
}

impl BookCheckoutList {
    pub fn hydrate(checkouts: Vec<BookCheckout>) -> Self {
        Self(checkouts)
    }

    pub fn do_checkout(&mut self, context: &AuditContext) -> Result<(), DomainError> {
        if self.is_checked_out() {
            return Err(DomainError::ValidationError(
                "Book is already checked out".to_string(),
            ));
        }

        self.0.push(BookCheckout::Active(CheckoutRecord {
            checkout_id: Uuid::new_v4(),
            checked_out_to: context.actor().into(),
            checked_out_at: context.timestamp(),
        }));

        Ok(())
    }

    pub fn do_return(&mut self, context: &AuditContext) -> Result<(), DomainError> {
        if let Some((idx, latest)) = self.latest_active_with_idx() {
            let actor = context.actor();
            if latest.checked_out_to.id() != actor.id() && !actor.is_admin() {
                return Err(DomainError::Forbidden);
            }

            self.0[idx] = BookCheckout::Returned {
                checkout: latest.clone(),
                returned_at: context.timestamp(),
            };

            Ok(())
        } else if self.is_returned() {
            Err(DomainError::ValidationError(
                "Book has already been returned".to_string(),
            ))
        } else {
            Err(DomainError::ValidationError(
                "Book is not currently checked out".to_string(),
            ))
        }
    }

    pub fn raw(&self) -> &[BookCheckout] {
        &self.0
    }

    fn latest(&self) -> Option<&BookCheckout> {
        self.0.iter().max_by_key(|checkout| match checkout {
            BookCheckout::Active(CheckoutRecord { checked_out_at, .. }) => checked_out_at,
            BookCheckout::Returned {
                checkout: CheckoutRecord { checked_out_at, .. },
                ..
            } => checked_out_at,
        })
    }
    fn latest_active_with_idx(&self) -> Option<(usize, &CheckoutRecord)> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(i, checkout)| match checkout {
                BookCheckout::Active(record) => Some((i, record)),
                _ => None,
            })
            .max_by_key(|(_, record)| record.checked_out_at)
    }
    fn is_checked_out(&self) -> bool {
        self.latest()
            .map(|checkout| matches!(checkout, BookCheckout::Active(_)))
            .unwrap_or(false)
    }
    fn is_returned(&self) -> bool {
        self.latest()
            .map(|checkout| matches!(checkout, BookCheckout::Returned { .. }))
            .unwrap_or(false)
    }
}

impl BookCheckout {
    pub fn hydrate(
        checkout_id: Uuid,
        checked_out_to: UserReference,
        checked_out_at: DateTime<Utc>,
        returned_at: Option<DateTime<Utc>>,
    ) -> Self {
        let checkout = CheckoutRecord {
            checkout_id,
            checked_out_to,
            checked_out_at,
        };
        match returned_at {
            Some(returned_at) => BookCheckout::Returned {
                checkout,
                returned_at,
            },
            None => BookCheckout::Active(checkout),
        }
    }

    pub fn id(&self) -> Uuid {
        match self {
            BookCheckout::Active(checkout) => checkout.checkout_id,
            BookCheckout::Returned { checkout, .. } => checkout.checkout_id,
        }
    }
    pub fn checked_out_to(&self) -> &UserReference {
        match self {
            BookCheckout::Active(checkout) => &checkout.checked_out_to,
            BookCheckout::Returned { checkout, .. } => &checkout.checked_out_to,
        }
    }
    pub fn checked_out_at(&self) -> DateTime<Utc> {
        match self {
            BookCheckout::Active(checkout) => checkout.checked_out_at,
            BookCheckout::Returned { checkout, .. } => checkout.checked_out_at,
        }
    }
    pub fn checked_out_by(&self) -> &UserReference {
        self.checked_out_to()
    }
    pub fn returned_at(&self) -> Option<DateTime<Utc>> {
        match self {
            BookCheckout::Active(_) => None,
            BookCheckout::Returned { returned_at, .. } => Some(*returned_at),
        }
    }
}
