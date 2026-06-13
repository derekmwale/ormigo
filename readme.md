https://crates.io/crates/ormigo


1. Set up your Rust project
    Make sure you have Rust installed.
    Check with:
    rustc --version
    cargo --version
    Make a new project (if you haven’t already):
    cargo new ormigo_test
    cd ormigo_test
    Add dependencies in Cargo.toml:
    [dependencies]
    tokio = { version = "1.41", features = ["full"] }
    sqlx = { version = "0.7", features = ["sqlite", "macros", "runtime-tokio-native-tls"] }
    serde = { version = "1.0", features = ["derive"] }
    thiserror = "1.0"
    Make sure your Cargo.toml matches what ormigo needs (sqlite + async support).
2. Create a test database
    Since this ORM is SQLite-first, create a simple test database.
    Open a terminal and run:
    sqlite3 test.db
    Create a table and insert some test data:
    CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        name TEXT,
        email TEXT,
        age INTEGER,
        is_active BOOLEAN
    );

    INSERT INTO users (name, email, age, is_active) VALUES
    ('Alice', 'alice@example.com', 25, 1),
    ('Bob', 'bob@example.com', 20, 0),
    ('Charlie', 'charlie@example.com', 30, 1);

    .exit
    This will give you three users in a users table.
3. Use main.rs to test
    Replace main.rs with the example usage you already have:
    #[tokio::main]
    async fn main() -> Result<(), ormigo::OrmigoError> {
        use ormigo::{OrmigoDB, QueryBuilder, User};

        let db = OrmigoDB::connect("sqlite://./test.db").await?;

        let users = db
            .fetch_all::<User>(
                QueryBuilder::new("users")
                    .filter("age > 21")
                    .filter("is_active = 1")
                    .select(&["id", "name", "email", "age", "is_active"]),
            )
            .await?;

        for user in users {
            println!("User: {:?}", user);
        }

        Ok(())
    }
    Run it:
    cargo run
    Expected output:
    User: User { id: 1, name: "Alice", email: "alice@example.com", age: 25, is_active: true }
    User: User { id: 3, name: "Charlie", email: "charlie@example.com", age: 30, is_active: true }
4. Writing automated tests
    Rust allows you to write tests inside your library. In src/lib.rs or a separate file tests/integration_test.rs:
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{OrmigoDB, QueryBuilder, User};

        #[tokio::test]
        async fn test_query_builder() {
            let query = QueryBuilder::new("users")
                .filter("age > 21")
                .select(&["id", "name"])
                .build();

            assert_eq!(query, "SELECT id, name FROM users WHERE age > 21");
        }

        #[tokio::test]
        async fn test_fetch_all() -> Result<(), OrmigoError> {
            let db = OrmigoDB::connect("sqlite://./test.db").await?;
            let users: Vec<User> = db
                .fetch_all(
                    QueryBuilder::new("users")
                        .filter("age > 21")
                        .filter("is_active = 1")
                        .select(&["id", "name", "email", "age", "is_active"]),
                )
                .await?;

            assert!(users.len() > 0);
            assert_eq!(users[0].age > 21, true);
            assert_eq!(users[0].is_active, true);

            Ok(())
        }
    }
    Run tests:
    cargo test
    test_query_builder → checks that the SQL string is built correctly.
    test_fetch_all → checks that the ORM can fetch data from SQLite.