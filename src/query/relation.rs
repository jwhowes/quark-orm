use crate::model::Model;

pub trait Relation<T: Clone>
where
    Self: AsRef<T> + AsMut<T> + Clone,
{
    fn cloned(&self) -> T;
}

#[derive(Clone)]
pub struct One<M: Model>(Box<M>);

impl<M: Model> AsRef<M> for One<M> {
    fn as_ref(&self) -> &M {
        self.0.as_ref()
    }
}

impl<M: Model> AsMut<M> for One<M> {
    fn as_mut(&mut self) -> &mut M {
        self.0.as_mut()
    }
}

impl<M: Model + Clone> Relation<M> for One<M> {
    fn cloned(&self) -> M {
        self.0.as_ref().clone()
    }
}

#[derive(Clone)]
pub struct MaybeOne<M: Model>(Box<Option<M>>);

impl<M: Model> AsRef<Option<M>> for MaybeOne<M> {
    fn as_ref(&self) -> &Option<M> {
        self.0.as_ref()
    }
}

impl<M: Model> AsMut<Option<M>> for MaybeOne<M> {
    fn as_mut(&mut self) -> &mut Option<M> {
        self.0.as_mut()
    }
}

impl<M: Model + Clone> Relation<Option<M>> for MaybeOne<M> {
    fn cloned(&self) -> Option<M> {
        self.0.as_ref().clone()
    }
}

#[derive(Clone)]
pub struct Many<M: Model>(Vec<M>);

impl<M: Model> AsRef<Vec<M>> for Many<M> {
    fn as_ref(&self) -> &Vec<M> {
        self.0.as_ref()
    }
}

impl<M: Model> AsMut<Vec<M>> for Many<M> {
    fn as_mut(&mut self) -> &mut Vec<M> {
        self.0.as_mut()
    }
}

impl<M: Model + Clone> Relation<Vec<M>> for Many<M> {
    fn cloned(&self) -> Vec<M> {
        self.0.clone()
    }
}
