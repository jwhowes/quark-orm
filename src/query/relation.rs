use std::marker::PhantomData;

use crate::model::Model;

pub struct One<M: Model>(PhantomData<M>);

pub struct Many<M: Model>(PhantomData<M>);

pub trait Relation<M: Model> {}
