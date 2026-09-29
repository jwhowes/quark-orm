use crate::error::Result;

pub trait SelectQuery<T>: Sized {
    fn find_first(self) -> impl Future<Output = Result<Option<T>>>;
    fn find_all(self) -> impl Future<Output = Result<Vec<T>>>;
}
