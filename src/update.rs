pub struct UpdateBuilder {
    table: String,
    updates: Vec<String>,
    filters: Vec<String>,
}

impl UpdateBuilder {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            updates: vec![],
            filters: vec![],
        }
    }

    pub fn set<T: ToString>(
        mut self,
        column: &str,
        value: T,
    ) -> Self {
        self.updates.push(format!(
            "{} = '{}'",
            column,
            value.to_string().replace("'", "''")
        ));

        self
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
             panic!("UpdateBuilder requires a WHERE clause");
        }
        
        let where_clause = if self.filters.is_empty() {
            "".to_string()
        } else {
            format!(
                " WHERE {}",
                self.filters.join(" AND ")
            )
        };

        format!(
            "UPDATE {} SET {}{}",
            self.table,
            self.updates.join(", "),
            where_clause
        )
    }
}