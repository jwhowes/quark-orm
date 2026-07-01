use sqlx::{Database, PgPool, Postgres, query::QueryAs};

use crate::{Result, model::Model};

pub mod builder;
pub mod relation;

pub trait ExecQuery<M: Model, O> {
    fn exec(self, db: &PgPool) -> impl Future<Output = Result<O>>;
}

pub struct FindFirst<'q, M: Model>(pub QueryAs<'q, Postgres, M, <Postgres as Database>::Arguments>);

impl<'q, M: Model> ExecQuery<M, Option<M>> for FindFirst<'q, M> {
    async fn exec(self, db: &PgPool) -> Result<Option<M>> {
        Ok(self.0.fetch_optional(db).await?)
    }
}

pub struct FindMany<'q, M: Model>(pub QueryAs<'q, Postgres, M, <Postgres as Database>::Arguments>);

impl<'q, M: Model> ExecQuery<M, Vec<M>> for FindMany<'q, M> {
    async fn exec(self, db: &PgPool) -> Result<Vec<M>> {
        Ok(self.0.fetch_all(db).await?)
    }
}

pub trait Select: Sized {
    fn as_clause(&self) -> String;
    fn where_clause(&self) -> Option<String>;
    fn join_clause(&self) -> Option<String>;

    fn select_query(self) -> String {
        let w = self.where_clause().unwrap_or("".to_string());
        let j = self.join_clause().unwrap_or("".to_string());

        format!("SELECT {}{}{}", self.as_clause(), w, j)
    }
}
