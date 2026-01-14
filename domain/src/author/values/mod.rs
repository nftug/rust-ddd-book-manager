use crate::define_id;

mod author_name;
mod author_reference;

pub use author_name::AuthorName;
pub use author_reference::AuthorReference;

define_id!(AuthorId);
