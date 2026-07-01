#[derive(Debug)]
pub struct DbError;

impl From<sqlx::Error> for DbError {
    // TODO
    fn from(_err: sqlx::Error) -> Self {
        Self
    }
}

pub type Result<T> = core::result::Result<T, DbError>;
