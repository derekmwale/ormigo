pub struct DeleteBuilder {
    table: String,
    filters: Vec<String>,
}

impl DeleteBuilder {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            filters: vec![],
        }
    }

    pub fn filter(
        mut self,
        condition: &str,
    ) -> Self {
        self.filters.push(condition.to_string());
        self
    }

    pub fn build(&self) -> String {

        if self.filters.is_empty() {
            panic!("DeleteBuilder requires a WHERE clause");
        }

        let where_clause = format!(
            " WHERE {}",
            self.filters.join(" AND ")
        );

        format!(
            "DELETE FROM {}{}",
            self.table,
            where_clause
        )
    }
}