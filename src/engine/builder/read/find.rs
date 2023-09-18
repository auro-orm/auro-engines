use crate::{
  engine::builder::{error::BuilderError, builder::QueryBuilder},
  parser::parsed::{FieldName, ParsedQuery},
};

pub fn unique(query_schema: &ParsedQuery) -> Result<String, BuilderError> {
  let builder = QueryBuilder::new(&query_schema)
    .order_fields(vec![FieldName::Select, FieldName::From, FieldName::Include, FieldName::Where])
    .build_ordered()?;

  Ok(builder)
}

pub fn many(query_schema: &ParsedQuery) -> Result<String, BuilderError> {
  let builder = QueryBuilder::new(&query_schema)
    .order_fields(vec![FieldName::Select, FieldName::From, FieldName::Where])
    .build_ordered()?;

  Ok(builder)
}
