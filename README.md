# Quark ORM

## Derive Model

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
	Book::find_first(filter! {
		id: id
	})
		.exec(db)
		.await
}
```

### Relations
```rust
use quark_orm::prelude::*;

#[derive(Model)]
#[quark_orm(table_name = "book")]
pub struct Book {
	#[quark_orm(primary_key)]
	pub id: i32,

	#[quark_orm(unique)]
	pub title: String,

	#[quark_orm(foreign_key = "review")]
	pub review_id: Option<i32>,

	pub review: MaybeOne<Review>,

	#[quark_orm(via = "book_author")]
	pub authors: Many<Author>
}

#[derive(Model)]
#[quark_orm(table_name = "review")]
pub struct Review {
	#[quark_orm(primary_key)]
	pub id: i32,

	pub rating: i32,

	pub book: One<Book>
}

#[derive(Model)]
#[quark_orm(table_name = "author")]
pub struct Author {
	#[quark_orm(primary_key)]
	pub id: i32,

	pub name: String,

	#[quark_orm(via = "book_author")]
	pub books: Many<Book>
}

#[derive(Model)]
#[quark_orm(
	table_name = "book_author",
	primary_key = ["book_id", "author_id"]
)]
pub struct BookAuthor {
	#[quark_orm(foreign_key = "book")]
	pub book_id: i32,

	#[quark_orm(foreign_key = "author")]
	pub author_id: i32,
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