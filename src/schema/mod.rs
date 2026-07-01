use std::env;

use dotenv::dotenv;
use sqlx::{PgPool, Postgres, QueryBuilder};

use crate::{
    error::Result,
    model::Model,
    query::ExecQuery,
    schema::book::{Book, book_model},
};

mod book;

fn get_pool() -> PgPool {
    let _ = dotenv();

    PgPool::connect_lazy(&env::var("DATABASE_URL").unwrap()).unwrap()
}

#[tokio::test]
async fn get_book() -> Result<()> {
    let db = get_pool();

    let book = Book::find_first(book_model::Select {
        filter: book_model::Filter {
            id: Some(1),
            title: None,
        },
        include: book_model::Include {},
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

#[tokio::test]
async fn get_book_with_review() -> Result<()> {
    let db = get_pool();

    let row = QueryBuilder::<Postgres>::new(
        "SELECT book.id AS \"book__id\" FROM book JOIN review ON book.id=review.book_id",
    )
    .build()
    .fetch_optional(&db)
    .await?
    .unwrap();

    println!("{:?}", row);

    Ok(())
}
