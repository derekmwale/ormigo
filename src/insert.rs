pub struct InsertBuilder {
    table: String,
    columns: Vec<String>,
    values: Vec<String>,
}

impl InsertBuilder {
    pub fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            columns: vec![],
            values: vec![],
        }
    }

    pub fn value<T: ToString>(
        mut self,
        column: &str,
        value: T,
    ) -> Self {
        self.columns.push(column.to_string());

        let value = format!(
            "'{}'",
            value.to_string().replace("'", "''")
        );

        self.values.push(value);

        self
    }

    pub fn build(&self) -> String {
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table,
            self.columns.join(", "),
            self.values.join(", ")
        )
    }
}