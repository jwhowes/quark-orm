use sqlx::{FromRow, Row, postgres::PgRow};

use crate::model::Model;

pub struct Book {
    pub id: i32,

    pub title: String,

    pub review_id: Option<i32>,
}

mod book_model {
    use sqlx::{Database, Postgres, query::QueryAs};

    use super::*;

    pub struct Filter {
        id: Option<i32>,
        title: Option<String>,
        // TODO: Relation filters
    }

    impl<'q> Into<QueryAs<'q, Postgres, Book, <Postgres as Database>::Arguments>> for Filter {
        fn into(self) -> QueryAs<'q, Postgres, Book, <Postgres as Database>::Arguments> {
            todo!()
        }
    }
}

impl<'r> FromRow<'r, PgRow> for Book {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("book.id")?,
            title: row.try_get("book.title")?,
            review_id: row.try_get("book.review_id")?,
        })
    }
}

impl Model for Book {
    type Filter = book_model::Filter;
}
