use std::env;

use dotenv::dotenv;
use sqlx::PgPool;

use crate::{
    error::Result,
    model::Model,
    query::ExecQuery,
    schema::book::{Book, book_model},
};

mod book;

#[tokio::test]
async fn get_book() -> Result<()> {
    let _ = dotenv();

    let db = PgPool::connect_lazy(&env::var("DATABASE_URL").unwrap()).unwrap();

    let book = Book::find_first(book_model::Filter {
        id: Some(1),
        title: None,
    })
    .exec(&db)
    .await?
    .unwrap();

    assert_eq!(
        book,
        Book {
            id: 1,
            title: "Test Book".to_string()
        }
    );

    Ok(())
}
