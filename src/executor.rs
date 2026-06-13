// ---------- src/executor.rs ----------

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use crate::builder::QueryBuilder;
use crate::insert::InsertBuilder;

use crate::error::OrmigoError;



pub struct OrmigoDB {
pub pool: SqlitePool,
}


impl OrmigoDB {
    pub async fn connect(database_url: &str) -> Result<Self, OrmigoError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
         Ok(Self { pool })
    }


    pub async fn fetch_all<T: for<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin>(
    &self,
    query: QueryBuilder,
    ) -> Result<Vec<T>, OrmigoError> {
        let sql = query.build();
        let result = sqlx::query_as::<_, T>(&sql)
            .fetch_all(&self.pool)
            .await?;
        Ok(result)
    }


    pub async fn execute_insert(
        &self,
        insert: InsertBuilder,
    ) -> Result<u64, OrmigoError> {

        let sql = insert.build();

        let result = sqlx::query(&sql)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub fn insert(
        &self,
        table: &str,
    ) -> InsertBuilder {
        InsertBuilder::new(table)
    }

    pub async fn execute_insert_returning_id(
        &self,
        insert: InsertBuilder,
    ) -> Result<i64, OrmigoError> {

        let sql = insert.build();

        let result = sqlx::query(&sql)
            .execute(&self.pool)
            .await?;

        Ok(result.last_insert_rowid())
    }


}
