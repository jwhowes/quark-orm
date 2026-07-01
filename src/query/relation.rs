use sqlx::{FromRow, Row, postgres::PgRow};

use crate::model::Model;

pub enum Relation<T> {
    NotIncluded,
    Included(T),
}

impl<T> Relation<T> {
    pub fn as_ref(&self) -> Relation<&T> {
        match self {
            Relation::NotIncluded => Relation::NotIncluded,
            Relation::Included(x) => Relation::Included(x),
        }
    }

    pub fn as_mut(&mut self) -> Relation<&mut T> {
        match self {
            Relation::NotIncluded => Relation::NotIncluded,
            Relation::Included(x) => Relation::Included(x),
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            Relation::NotIncluded => panic!(),

            Relation::Included(x) => x,
        }
    }
}

impl<T> Default for Relation<T> {
    fn default() -> Self {
        Relation::NotIncluded
    }
}

impl<'q, M: Model> FromRow<'q, PgRow> for Relation<M> {
    fn from_row(row: &'q PgRow) -> Result<Self, sqlx::Error> {
        match row.try_get::<Option<i32>, _>(M::ID)? {
            None => Ok(Relation::NotIncluded),

            Some(_) => Ok(Relation::Included(M::from_row(row)?)),
        }
    }
}

pub type One<T> = Relation<Box<T>>;
pub type MaybeOne<T> = Relation<Option<Box<T>>>;
pub type Many<T> = Relation<Vec<T>>;
