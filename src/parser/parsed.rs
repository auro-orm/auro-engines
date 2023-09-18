use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ParsedQuery {
  pub table: String,
  pub schema: String,
  pub fields: Vec<ParsedField>,
  pub options: QueryOptions,
}

impl ParsedQuery {
  pub fn new(table: String, schema: String, fields: Vec<ParsedField>, options: QueryOptions) -> Self {
    Self {
      table,
      schema,
      fields,
      options,
    }
  }

  pub fn get_field_by_field_name(&self, name: FieldName) -> Option<&ParsedField> {
    self.fields.iter().find(|field| field.name == name)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldName {
  Where,
  Select,
  Include,
  Data,
  Set,
  Return,
  NoReturn,
  Aggs,
  From
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedField {
  pub name: FieldName,
  pub arguments: Vec<ParsedArgument>,
}

pub(crate) trait FieldListLookup {
  fn lookup(&mut self, name: &str) -> Option<ParsedField>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedArgument {
  pub name: String,
  pub value: Option<ParsedValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParsedValue {
  String(String),
  Number(String),
  Boolean(String),
  Float(String),
  Date(String),
  DateTime(String),
  Custom(String),
  Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedInclude {
  pub joins: Vec<(String, String, String, String)>
}

#[derive(Debug, Clone, PartialEq)]
pub struct QueryOptions {
  pub order_by: Option<HashMap<String, String>>,
  pub limit: Option<String>,
  pub offset: Option<String>,
  pub num_of_rows: Option<usize>,
  pub include: Option<ParsedInclude>,
  pub group_by: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub enum ParsedOperation {
  FindFirst,
  FindMany,
  UpdateOne,
  UpdateMany,
  DeleteOne,
  DeleteMany,
  InsertOne,
  InsertMany,
  Count,
  Average,
}

#[derive(Debug, Clone)]
pub struct ParsedStatement {
  pub query: ParsedQuery,
  pub operation: ParsedOperation,
}
