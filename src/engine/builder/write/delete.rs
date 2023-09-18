use crate::{
  engine::builder::{error::BuilderError, builder::QueryBuilder},
  parser::parsed::{self, FieldName},
};

pub fn unique(query_schema: &parsed::ParsedQuery) -> Result<String, BuilderError> {
  let builder = QueryBuilder::new(&query_schema)
    .order_fields(vec![FieldName::From, FieldName::Where, FieldName::Return])
    .build_ordered()?;
 
  Ok(format!("DELETE FROM {}", builder))
}

pub fn many(query_schema: &parsed::ParsedQuery) -> Result<String, BuilderError> {
  let builder = QueryBuilder::new(&query_schema)
    .order_fields(vec![FieldName::From, FieldName::Where, FieldName::Return])
    .build_ordered()?;

  Ok(format!("DELETE FROM {}", builder))
}