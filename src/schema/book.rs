use sqlx::Row;

use crate::model::Model;

#[derive(Debug, PartialEq)]
pub struct Book {
    pub id: i32,

    pub title: String,
}

pub mod book_model {
    use sqlx::{Database, Postgres, QueryBuilder, query::Query};

    pub struct Filter {
        pub id: Option<i32>,
        pub title: Option<String>,
    }

    impl<'q> Into<Query<'q, Postgres, <Postgres as Database>::Arguments>> for Filter {
        fn into(self) -> Query<'q, Postgres, <Postgres as Database>::Arguments> {
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

            sqlx::query(
                QueryBuilder::<Postgres>::new(format!("SELECT * FROM book {}", where_clause)).sql(),
            )
        }
    }
}

impl Model for Book {
    type Filter = book_model::Filter;

    fn from_row(row: sqlx::postgres::PgRow) -> crate::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
        })
    }
}
