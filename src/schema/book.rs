use sqlx::{FromRow, Row, postgres::PgRow};

use crate::model::Model;

#[derive(Debug, PartialEq)]
pub struct Book {
    pub id: i32,

    pub title: String,
}

pub mod book_model {
    use crate::query::Select as SelectTrait;

    use super::*;

    pub struct Select {
        pub filter: Filter,
        pub include: Include,
    }

    pub struct Filter {
        pub id: Option<i32>,
        pub title: Option<String>,
    }

    pub struct Include {}

    impl SelectTrait for Select {
        fn as_clause(&self) -> String {
            Book::AS_CLAUSE.to_string()
        }

        fn join_clause(&self) -> Option<String> {
            None
        }

        fn where_clause(&self) -> Option<String> {
            let conds = [
                self.filter.id.map(|x| format!("book__id={}", x)),
                self.filter
                    .title
                    .as_ref()
                    .map(|x| format!("book__title=\"{}\"", x)),
            ]
            .into_iter()
            .filter_map(|x| x)
            .collect::<Vec<_>>();

            if conds.is_empty() {
                None
            } else {
                Some(format!(" WHERE {}", conds.join(" AND ")))
            }
        }
    }
}

impl Model for Book {
    const ID: &'static str = "book__id";
    const AS_CLAUSE: &'static str = "book.id  AS \"book__id\", book.title AS \"book__title\"";

    type Select = book_model::Select;
}

impl<'r> FromRow<'r, PgRow> for Book {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("book__id")?,

            title: row.try_get("book__title")?,
        })
    }
}
