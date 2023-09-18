use crate::{
  engine::builder::{error::BuilderError, builder::QueryBuilder},
  parser::parsed::{self, FieldName},
};

pub fn one(query_schema: &parsed::ParsedQuery) -> Result<String, BuilderError> {
  let builder = QueryBuilder::new(&query_schema)
    .order_fields(vec![FieldName::From, FieldName::Data, FieldName::Return])
    .build_ordered()?;

  Ok(format!("INSERT INTO {}", builder))
}

pub fn many(query_schema: &parsed::ParsedQuery) -> Result<String, BuilderError> {
  let builder = QueryBuilder::new(&query_schema)
    .order_fields(vec![FieldName::From, FieldName::Data, FieldName::Return])
    .build_ordered()?;

  Ok(format!("INSERT INTO {}", builder))
}