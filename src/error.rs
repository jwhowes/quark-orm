pub struct DbError;

impl From<sqlx::Error> for DbError {
    fn from(_err: sqlx::Error) -> Self {
        Self
    }
}
