use crate::{query::relation::Many, schema::book::Book};

pub struct Author {
    pub id: i32,

    pub name: String,

    pub books: Many<Book>,
}
