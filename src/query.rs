use sqlx::PgPool;

use crate::{Result, model::Model};

pub trait ExecQuery<M: Model, O> {
    fn exec(self, db: &PgPool) -> impl Future<Output = Result<O>>;
}

pub struct FindFirst<M: Model>(pub M::Filter);

impl<M: Model> ExecQuery<M, Option<M>> for FindFirst<M> {
    async fn exec(self, db: &PgPool) -> Result<Option<M>> {
        Ok(M::select(self.0).fetch_optional(db).await?)
    }
}

pub struct FindMany<M: Model>(pub M::Filter);

impl<M: Model> ExecQuery<M, Vec<M>> for FindMany<M> {
    async fn exec(self, db: &PgPool) -> Result<Vec<M>> {
        Ok(M::select(self.0).fetch_all(db).await?)
    }
}
