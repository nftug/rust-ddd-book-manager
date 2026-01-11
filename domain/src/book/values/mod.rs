pub mod book_author;
pub mod book_description;
pub mod book_isbn;
pub mod book_owner;
pub mod book_title;

use crate::define_id;

pub use book_author::BookAuthor;
pub use book_description::BookDescription;
pub use book_isbn::BookIsbn;
pub use book_owner::BookOwner;
pub use book_title::BookTitle;

define_id!(BookId);
