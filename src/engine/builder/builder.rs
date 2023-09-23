use super::{error::BuilderError, fields, options::build_options};
use crate::{
  constants::{constants, errors},
  parser::parsed::{FieldName, ParsedQuery},
};

pub struct QueryBuilder<'a> {
  query: &'a ParsedQuery,
  field_order: Vec<FieldName>,
}

impl<'a> QueryBuilder<'a> {
  pub fn new(query: &'a ParsedQuery) -> Self {
    QueryBuilder {
      query,
      field_order: Vec::new(),
    }
  }

  pub fn order_fields(&mut self, field_order: Vec<FieldName>) -> &mut Self {
    self.field_order = field_order;
    self
  }

  fn build_aggregate(&self) -> Result<String, BuilderError> {
    if let Some(aggregate_field) = self.query.fields.iter().find(|field| field.name == FieldName::Aggs) {
      fields::build_aggregate(aggregate_field)
    } else {
      Ok(String::new())
    }
  }

  fn build_select(&self) -> Result<String, BuilderError> {
    if let Some(select_field) = self.query.fields.iter().find(|field| field.name == FieldName::Select) {
      fields::build_select(select_field)
    } else {
      Ok(format!("{} FROM", constants::SELECT_ALL))
    }
  }

  fn build_where(&self) -> Result<String, BuilderError> {
    if let Some(where_field) = self.query.fields.iter().find(|field| field.name == FieldName::Where) {
      fields::build_where(where_field)
    } else {
      Ok(String::new())
    }
  }

  fn build_set(&self) -> Result<String, BuilderError> {
    if let Some(set_field) = self.query.fields.iter().find(|field| field.name == FieldName::Set) {
      fields::build_set(set_field)
    } else {
      Ok(String::new())
    }
  }

  fn build_data_one(&self) -> Result<String, BuilderError> {
    if let Some(data_field) = self.query.fields.iter().find(|field| field.name == FieldName::Data) {
      fields::build_one_data(data_field)
    } else {
      Ok(String::new())
    }
  }

  fn build_from(&self) -> Result<String, BuilderError> {
    if self.query.schema.is_empty() {
      return Err(BuilderError::MissingSchema(errors::MISSING_SCHEMA));
    }
    if self.query.table.is_empty() {
      return Err(BuilderError::MissingTable(errors::MISSING_TABLE));
    }

    let from_clause = format!("{}.{}", self.query.schema, self.query.table);
    Ok(from_clause)
  }

  fn build_options(&self) -> Result<String, BuilderError> {
    let options = build_options(self.query.options.clone())?;
    Ok(options)
  }

  fn build_return(&self) -> Result<String, BuilderError> {
    if let Some(return_field) = self.query.fields.iter().find(|field| field.name == FieldName::Return) {
      fields::build_return(return_field)
    } else if self.query.fields.iter().any(|field| field.name == FieldName::NoReturn) {
      Ok(String::new())
    } else {
      Ok(String::from(constants::RETURN_ALL))
    }
  }

  fn build_data_many(&self) -> Result<String, BuilderError> {
    let data_field = match self.query.fields.iter().find(|field| field.name == FieldName::Data) {
      Some(field) => field,
      None => return Err(BuilderError::InvalidStatement(errors::INVALID_STATEMENT)),
    };

    let num_rows = match self.query.options.num_of_rows {
      Some(num_of_rows) => num_of_rows,
      None => return Err(BuilderError::InvalidStatement(errors::INVALID_STATEMENT)),
    };

    if data_field.arguments.is_empty() {
      return Err(BuilderError::InvalidStatement(errors::INVALID_STATEMENT));
    }

    fields::build_many_data(data_field, num_rows)
  }

  fn build_data(&self) -> Result<String, BuilderError> {
    let num_rows = self.query.options.num_of_rows.unwrap_or(1);

    if num_rows > 1 {
      self.build_data_many()
    } else {
      self.build_data_one()
    }
  }

  fn build_include(&self) -> Result<String, BuilderError> {
    if let Some(include_field) = &self.query.options.include {
      fields::build_include(include_field, &self.query.schema)
    } else {
      Ok(String::new())
    }
  }

  pub fn build_ordered(&self) -> Result<String, BuilderError> {
    let mut ordered_fields = Vec::new();

    for name in &self.field_order {
      match name {
        FieldName::Select => ordered_fields.push(self.build_select()?),
        FieldName::Include => ordered_fields.push(self.build_include()?),
        FieldName::From => ordered_fields.push(self.build_from()?),
        FieldName::Where => ordered_fields.push(self.build_where()?),
        FieldName::Set => ordered_fields.push(self.build_set()?),
        FieldName::Data => ordered_fields.push(self.build_data()?),
        FieldName::Return => ordered_fields.push(self.build_return()?),
        FieldName::Aggs => ordered_fields.push(self.build_aggregate()?),
        _ => return Err(BuilderError::InvalidFieldName(errors::INVALID_FIELD)),
      }
    }

    ordered_fields.push(self.build_options()?);

    Ok(ordered_fields.join(" "))
  }
}
