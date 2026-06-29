use sqlx::PgPool;

use crate::{error::Result, model::Model};

pub mod relation;

pub trait ExecQuery<M: Model, O> {
    fn exec(self, db: &PgPool) -> impl Future<Output = Result<O>>;
}

pub struct FindFirst<M: Model>(M::Filter);

impl<M: Model> ExecQuery<M, Option<M>> for FindFirst<M> {
    async fn exec(self, db: &PgPool) -> Result<Option<M>> {
        Ok(self.0.into().fetch_optional(db).await?)
    }
}

pub struct FindMany<M: Model>(M::Filter);

impl<M: Model> ExecQuery<M, Vec<M>> for FindMany<M> {
    async fn exec(self, db: &PgPool) -> Result<Vec<M>> {
        Ok(self.0.into().fetch_all(db).await?)
    }
}
