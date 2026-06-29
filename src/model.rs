use sqlx::{Database, FromRow, Postgres, postgres::PgRow, query::QueryAs};

use crate::query::relation::Relation;

pub trait Model
where
    Self: for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    type Filter: for<'q> Into<QueryAs<'q, Postgres, Self, <Postgres as Database>::Arguments>>;

    type With<R: Relation<Self>>;
}
