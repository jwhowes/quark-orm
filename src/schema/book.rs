use sqlx::{FromRow, Row, postgres::PgRow};

use crate::model::Model;

#[derive(Debug, PartialEq)]
pub struct Book {
    pub id: i32,

    pub title: String,
}

pub mod book_model {
    use sqlx::{
        Database, Postgres, QueryBuilder,
        query::{Query, QueryAs},
    };

    use crate::{model::Model, schema::book::Book};

    pub struct Filter {
        pub id: Option<i32>,
        pub title: Option<String>,
    }

    impl<'q> Into<QueryAs<'q, Postgres, Book, <Postgres as Database>::Arguments>> for Filter {
        fn into(self) -> QueryAs<'q, Postgres, Book, <Postgres as Database>::Arguments> {
            let conds = [
                self.id.map(|x| format!("id={}", x)),
                self.title.map(|x| format!("title='{}'", x)),
            ]
            .into_iter()
            .filter_map(|x| x)
            .collect::<Vec<_>>()
            .join(" AND ");

            let where_clause = if conds.is_empty() {
                "".to_string()
            } else {
                format!(" WHERE {}", conds)
            };

            sqlx::query_as(
                QueryBuilder::<Postgres>::new(format!(
                    "SELECT {} FROM book {}",
                    Book::AS_CLAUSE,
                    where_clause
                ))
                .sql(),
            )
        }
    }
}

impl Model for Book {
    const ID: &'static str = "book__id";
    const AS_CLAUSE: &'static str = "book.id  AS \"book__id\", book.title AS \"book__title\"";

    type Filter = book_model::Filter;
}

impl<'r> FromRow<'r, PgRow> for Book {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("book__id")?,

            title: row.try_get("book__title")?,
        })
    }
}
