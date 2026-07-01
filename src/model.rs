use sqlx::{FromRow, Postgres, QueryBuilder, postgres::PgRow};

use crate::query::{FindFirst, FindMany, Select};

/* SELECT {as_clause} FROM book WHERE {where_clause} {join_clause} */

pub trait Model: Send + Unpin
where
    Self: for<'q> FromRow<'q, PgRow>,
{
    const ID: &'static str;
    const AS_CLAUSE: &'static str;

    type Select: Select;

    fn find_first(s: Self::Select) -> FindFirst<'static, Self> {
        FindFirst(sqlx::query_as(
            QueryBuilder::<Postgres>::new(s.select_query()).sql(),
        ))
    }

    fn find_many(s: Self::Select) -> FindMany<'static, Self> {
        FindMany(sqlx::query_as(
            QueryBuilder::<Postgres>::new(s.select_query()).sql(),
        ))
    }
}
