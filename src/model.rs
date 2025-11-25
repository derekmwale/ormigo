// // ---------- main.rs (example usage) ----------

// #[tokio::main]
// async fn main() -> Result<(), ormigo::OrmigoError> {
// use ormigo::{OrmigoDB, QueryBuilder, User};


// let db = OrmigoDB::connect("sqlite://./example.db").await?;


// let users = db
// .fetch_all::<User>(
// QueryBuilder::new("users")
// .filter("age > 21")
// .filter("is_active = 1")
// .select(&["id", "name", "email", "age", "is_active"]),
// )
// .await?;


// for user in users {
// println!("User: {:?}", user);
// }


// Ok(())
// }