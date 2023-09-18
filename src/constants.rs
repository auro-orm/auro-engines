pub mod constants {
  pub const CURSOR: &str = "CURSOR ";
  pub const TAKE: &str = "TAKE ";
  pub const SKIP: &str = "SKIP ";
  pub const LIMIT: &str = " LIMIT ";
  pub const OFFSET: &str = " OFFSET ";
  pub const ORDER_BY: &str = "ORDER BY ";
  pub const ASC: &str = " ASC";
  pub const DESC: &str = " DESC";
  pub const WHERE: &str = "WHERE";
  pub const AND: &str = " AND ";
  pub const OR: &str = " OR ";
  pub const NOT: &str = " NOT ";
  pub const IN: &str = " IN ";
  pub const NOT_IN: &str = " NOT IN ";
  pub const SELECT: &str = "SELECT";
  pub const FROM: &str = " FROM";
  pub const INTO: &str = " INTO";
  pub const VALUES: &str = " VALUES";
  pub const SET: &str = "SET";
  pub const SELECT_ALL: &str = "SELECT *";
  pub const RETURN_ALL: &str = "RETURNING *";
  pub const RETURN: &str = "RETURNING ";
  pub const GROUP_BY: &str = "GROUP BY ";
}

pub mod errors {
  pub const MISSING_SCHEMA: &str = "Schema is missing.";
  pub const MISSING_TABLE: &str = "Table is missing.";
  pub const INVALID_FIELD: &str = "Invalid field name.";
  pub const INVALID_STATEMENT: &str = "Invalid statement.";
}
