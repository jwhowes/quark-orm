use sqlx::PgPool;

use crate::{Result, model::Model};

pub mod relation;

pub trait ExecQuery<M: Model, O> {
    fn exec(self, db: &PgPool) -> impl Future<Output = Result<O>>;
}

pub struct FindFirst<M: Model>(pub M::Filter);

impl<M: Model> ExecQuery<M, Option<M>> for FindFirst<M> {
    async fn exec(self, db: &PgPool) -> Result<Option<M>> {
        self.0
            .into()
            .fetch_optional(db)
            .await?
            .map(M::from_row)
            .map_or(Ok(None), |row| row.map(Some))
    }
}

pub struct FindMany<M: Model>(pub M::Filter);

impl<M: Model> ExecQuery<M, Vec<M>> for FindMany<M> {
    async fn exec(self, db: &PgPool) -> Result<Vec<M>> {
        self.0
            .into()
            .fetch_all(db)
            .await?
            .into_iter()
            .map(M::from_row)
            .collect()
    }
}
