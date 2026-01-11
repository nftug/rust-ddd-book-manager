pub mod user_email;
pub mod user_name;
pub mod user_reference;

use crate::define_id;

pub use user_email::UserEmail;
pub use user_name::UserName;
pub use user_reference::UserReference;

define_id!(UserId);
