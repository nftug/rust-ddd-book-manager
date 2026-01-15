mod book_author_list;
mod book_checkout;
mod book_description;
mod book_isbn;
mod book_owner;
mod book_title;

use crate::define_id;

pub use book_author_list::*;
pub use book_checkout::*;
pub use book_description::BookDescription;
pub use book_isbn::BookIsbn;
pub use book_owner::BookOwner;
pub use book_title::BookTitle;

define_id!(BookId);
