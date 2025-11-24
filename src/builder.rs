// ---------- src/builder.rs ----------

pub struct QueryBuilder {
    pub table: String,
    pub filters: Vec<String>,
    pub selects: Vec<String>,
    }
    
    
    impl QueryBuilder {
    pub fn new(table: &str) -> Self {
    Self {
    table: table.to_string(),
    filters: vec![],
    selects: vec![],
    }
    }
    
    
    pub fn filter(mut self, condition: &str) -> Self {
    self.filters.push(condition.to_string());
    self
    }
    
    
    pub fn select(mut self, fields: &[&str]) -> Self {
    self.selects = fields.iter().map(|f| f.to_string()).collect();
    self
    }
    
    
    pub fn build(&self) -> String {
    let select = if self.selects.is_empty() {
    "*".to_string()
    } else {
    self.selects.join(", ")
    };
    
    
    let where_clause = if self.filters.is_empty() {
    "".to_string()
    } else {
    format!(" WHERE {}", self.filters.join(" AND "))
    };
    
    
    format!("SELECT {} FROM {}{}", select, self.table, where_clause)
    }
}