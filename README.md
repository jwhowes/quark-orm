# Quark ORM

## Create a Schema
```rust
use quark_orm::prelude::*;

#[derive(Model)]
pub struct Author {
	#[quark_orm(primary_key, auto_increment)]
	pub id: u32,

	#[quark_orm(unique)]
	pub name: String,

	pub books: Many<Author>
}

#[derive(Model)]
pub struct Book {
	#[quark_orm(primary_key, auto_increment)]
	pub id: u32,

	pub title: String,

	pub authors: One<Author>
}
```

## Query a Database
```rust
use quark_orm::{Result, prelude::*};
use crate::schema::{Book, Author};

pub async fn get_book_by_id(id: u32) -> Result<Option<Book>> {
	Book::find_by_id(id)
		.exec()
		.await
}

pub async fn find_author_books(name: &str) -> Result<Vec<Book>> {
	Ok(
		Author::find_by_name(name)
			.with(Author::With::Book)
			.exec()
			.await?
			.map(|a| a.books)
			.unwrap_or_default
	)
}
```

## Migrations
```bash
# Initialize the migrations state
# {migrations}: the directory to store migrations (default = 'migrations')
quark migrate init --dir=migrations

# Display current status of migrations
quark migrate status

# Migrate up to and including a given migration
# {to}: the migration to stop at (optional, default = most recent)
# {force}: if true then drops data without warning (flag, default = false)
quark migrate up --to --force

# Create a migration based on the current code schema
# {no-write}: if true then the migration isn't run (flag, default = false)
# {force}: if true then drops data without warning (flag, default = false)
quark migrate create --no-write --force

# Drops the schema and re-runs to a given migration
# {to}: the migration to stop at (optional. default = most recent)
# {force}: if true then runs without warning (flag, default = false)
quark migrate reset --to --force
```