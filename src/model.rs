use sqlx::{Database, FromRow, Postgres, postgres::PgRow, query::QueryAs};

use crate::query::{FindFirst, FindMany};

pub trait Model: Send + Unpin
where
    Self: for<'q> FromRow<'q, PgRow>,
{
    const ID: &'static str;
    const AS_CLAUSE: &'static str;

    type Filter: for<'q> Into<QueryAs<'q, Postgres, Self, <Postgres as Database>::Arguments>>;

    fn find_first(q: Self::Filter) -> FindFirst<Self> {
        FindFirst(q)
    }

    fn find_many(q: Self::Filter) -> FindMany<Self> {
        FindMany(q)
    }
}
