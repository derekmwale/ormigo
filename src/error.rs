// ---------- src/error.rs ----------
use thiserror::Error;


#[derive(Error, Debug)]
pub enum OrmigoError {
#[error("Database error: {0}")]
SqlxError(#[from] sqlx::Error),


#[error("Other error: {0}")]
Other(String),
}