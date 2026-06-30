use sqlx::{Database, FromRow, Postgres, postgres::PgRow, query::QueryAs};

use crate::query::{FindFirst, FindMany};

pub trait Model
where
    Self: for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    type Filter: for<'q> Into<QueryAs<'q, Postgres, Self, <Postgres as Database>::Arguments>>;

    fn find_first(q: Self::Filter) -> FindFirst<Self> {
        FindFirst(q)
    }

    fn find_many(q: Self::Filter) -> FindMany<Self> {
        FindMany(q)
    }
}
