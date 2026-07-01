mod error;
mod model;
mod query;

pub mod prelude;

pub use error::Result;
pub use query::relation::Relation;

#[cfg(test)]
mod schema;
