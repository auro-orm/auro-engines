use crate::{
  engine::builder::{error::BuilderError, builder::QueryBuilder},
  parser::parsed::{FieldName, ParsedQuery},
};

pub fn count(query_schema: &ParsedQuery) -> Result<String, BuilderError> {
  let builder = QueryBuilder::new(&query_schema)
    .order_fields(vec![FieldName::From, FieldName::Where])
    .build_ordered()?;

  Ok(format!("SELECT COUNT (*) FROM {}", builder))
}

pub fn average(query_schema: &ParsedQuery) -> Result<String, BuilderError> {
  let builder = QueryBuilder::new(&query_schema)
    .order_fields(vec![FieldName::From, FieldName::Where])
    .build_ordered()?;

  Ok(format!("SELECT AVG(id) FROM {}", builder))
}
