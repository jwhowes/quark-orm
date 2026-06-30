use crate::{query::relation::One, schema::book::Book};

pub struct Review {
    pub id: i32,

    pub rating: i32,

    pub book: One<Book>,
}
