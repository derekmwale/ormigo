// ---------- src/lib.rs ----------
pub mod builder;
pub mod insert;

pub mod executor;
pub mod model;
pub mod error;

pub use insert::InsertBuilder;
pub use builder::QueryBuilder;

pub use executor::OrmigoDB;
pub use model::*;
pub use error::OrmigoError;