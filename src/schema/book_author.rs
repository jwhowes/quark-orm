use crate::{
    query::relation::One,
    schema::{author::Author, book::Book},
};

pub struct BookAuthor {
    pub book_id: i32,
    pub author_id: i32,

    pub book: One<Book>,
    pub author: One<Author>,
}
