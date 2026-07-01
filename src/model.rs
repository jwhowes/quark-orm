use sqlx::{Database, Postgres, postgres::PgRow, query::Query};

use crate::{
    error::Result,
    query::{FindFirst, FindMany},
};

pub trait Model: Sized {
    type Filter: for<'q> Into<Query<'q, Postgres, <Postgres as Database>::Arguments>>;

    fn find_first(q: Self::Filter) -> FindFirst<Self> {
        FindFirst(q)
    }

    fn find_many(q: Self::Filter) -> FindMany<Self> {
        FindMany(q)
    }

    fn from_row(row: PgRow) -> Result<Self>;
}
