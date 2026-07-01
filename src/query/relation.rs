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

pub type One<T> = Relation<Box<T>>;
pub type MaybeOne<T> = Relation<Option<Box<T>>>;
pub type Many<T> = Relation<Vec<T>>;
