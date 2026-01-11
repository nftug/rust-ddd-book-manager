mod user_email;
mod user_name;
mod user_reference;

use crate::define_id;

pub use user_email::UserEmail;
pub use user_name::UserName;
pub use user_reference::UserReference;

define_id!(UserId);
