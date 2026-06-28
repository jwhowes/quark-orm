# Quark ORM

## Derive Model

### Basic
```rust
use quark_orm::prelude::*;

#[derive(Model)]
#[quark_orm(table_name = "book")]
pub struct Book {
	#[quark_orm(primary_key)]
	pub id: i32,

	#[quark_orm(unique)]
	pub title: String,

	pub rating: Optional<i32>
}

async fn get_book_by_id(id: i32, db: &PgPool) -> quark_orm::Result<Book> {
	Book::find_first(query! {
		filter: {
			id: id
		}
	})
		.exec(db)
		.await
}
```

### One-to-one Relation
```rust
use quark_orm::prelude::*;

#[derive(Model)]
#[quark_orm(table_name = "book")]
pub struct Book {
	#[quark_orm(primary_key)]
	pub id: i32,

	#[quark_orm(foreign_key = "book_review")]
	pub review_id: Option<i32>,

	#[quark_orm(unique)]
	pub title: String,

	pub review Option<Review>
}

#[derive(Model)]
#[quark_orm(table_name = "book_review")]
pub struct BookReview {
	#[quark_orm(primary_key)]
	pub id: i32,

	#[quark_orm(foreign_key = "book")]
	pub book_id: i32,

	pub rating: i32,

	// Note: the existence of a book for a BookReview is non-optional, but the relation being returned as part of the model is
	// TODO: think this through
	pub book: Option<Book>
}

async fn get_book_with_review(id: i32, db: &PgPool) -> quark_orm::Result<Book> {
	Book::find_first(query! {
		filter: {
			id: id
		},
		with: [review]
	})
}
```

## Migrations
```bash
# Initiate migrations (`migrations-dir` defaults to "migrations")
quark_orm migrate init --migrations-dir

# Create a migration (if `name` isn't provided you'll be prompted to provide one). Warns before dropping data
quark_orm migrate create --name

# Run pending migrations. Warns before dropping data. May fail
quark_orm migrate up

# Prints the status of all migrations: PENDING, COMPLETE, or FAILED (with failure reason)
quark_orm migrate status
```