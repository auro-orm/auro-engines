use super::{
  error::ParserError,
  parsed::{
    FieldName, ParsedArgument, ParsedField, ParsedInclude, ParsedOperation, ParsedQuery, ParsedStatement, ParsedValue,
    QueryOptions,
  },
  utils,
};
use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};
use napi_derive::napi;
use std::{collections::HashMap, str::FromStr};

#[napi(object)]
#[derive(Debug, Clone, PartialEq)]
pub struct IncludeField {
  pub joins: Vec<Join>,
}

#[napi(object)]
#[derive(Debug, Clone, PartialEq)]
pub struct Join {
  pub table: String,
  pub key: String,
  pub joining_table: String,
  pub joining_key: String,
}

impl IncludeField {
  pub fn parse(&self) -> ParsedInclude {
    let joins = self
      .joins
      .iter()
      .map(|join| {
        (
          join.table.clone(),
          join.key.clone(),
          join.joining_table.clone(),
          join.joining_key.clone(),
        )
      })
      .collect();

    ParsedInclude { joins }
  }
}
#[napi(object)]
#[derive(Debug)]
pub struct Field {
  pub name: String,
  pub arguments: Vec<Argument>,
}

impl Field {
  pub fn parse(&self) -> Result<ParsedField, ParserError> {
    let name = self.parse_field_name()?;
    let arguments = self.parse_arguments()?;

    Ok(ParsedField { name, arguments })
  }

  fn parse_field_name(&self) -> Result<FieldName, ParserError> {
    FieldName::from_str(&self.name).map_err(|err| err)
  }

  fn parse_arguments(&self) -> Result<Vec<ParsedArgument>, ParserError> {
    self
      .arguments
      .iter()
      .map(|argument| argument.parse())
      .collect::<Result<Vec<_>, _>>()
  }
}

impl FromStr for FieldName {
  type Err = ParserError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "where" => Ok(FieldName::Where),
      "select" => Ok(FieldName::Select),
      "include" => Ok(FieldName::Include),
      "data" => Ok(FieldName::Data),
      "set" => Ok(FieldName::Set),
      "return" => Ok(FieldName::Return),
      "no_return" => Ok(FieldName::NoReturn),
      "aggs" => Ok(FieldName::Aggs),
      _ => Err(ParserError::InputError("Invalid field name!".to_string())),
    }
  }
}

#[napi(object)]
#[derive(Debug)]
pub struct Argument {
  pub name: String,
  pub value: Option<String>,
  pub value_type: Option<String>,
}

impl Argument {
  pub fn parse(&self) -> Result<ParsedArgument, ParserError> {
    let value = match self.parse_value() {
      Ok(value) => value,
      Err(err) => return Err(err),
    };

    let name = utils::camel_to_snake_case(&self.name);

    Ok(ParsedArgument { name, value })
  }

  fn parse_value(&self) -> Result<Option<ParsedValue>, ParserError> {
    let value = match &self.value {
      Some(str) => str.to_string(),
      None => return Ok(None),
    };

    let value_type = match &self.value_type {
      Some(value_type) => value_type.as_str(),
      None => return Ok(None),
    };

    match value_type {
      "number" => return Ok(Some(ParsedValue::Number(value))),
      "string" => return Ok(Some(ParsedValue::String(value))),
      "boolean" => return Ok(Some(ParsedValue::Boolean(value))),
      "date" => return Ok(Some(ParsedValue::Date(value))),
      "datetime" => return Ok(Some(ParsedValue::DateTime(value))),
      "custom" => return Ok(Some(ParsedValue::Custom(value))),
      "null" => return Ok(Some(ParsedValue::Null)),
      _ => Err(ParserError::ParseError("Invalid value type!".to_string())),
    }
  }
}

#[napi(object)]
#[derive(Debug, Clone)]
pub struct OrderBy {
  pub field: String,
  pub order: Order,
}

impl OrderBy {
  pub fn parse(order_by: Option<Vec<OrderBy>>) -> Option<HashMap<String, String>> {
    order_by.map(|vec| {
      vec.into_iter().fold(HashMap::new(), |mut acc, order_by| {
        acc.insert(order_by.field.clone(), order_by.order.parse().to_string());
        acc
      })
    })
  }
}

#[napi]
#[derive(Debug)]
pub enum Order {
  Asc,
  Desc,
}

impl Order {
  pub fn parse(&self) -> &str {
    match self {
      Order::Asc => "asc",
      Order::Desc => "desc",
    }
  }
}

#[napi(object)]
#[derive(Debug)]
pub struct Options {
  pub order_by: Option<Vec<OrderBy>>,
  pub limit: Option<i64>,
  pub offset: Option<i64>,
  pub num_of_rows: Option<i64>,
  pub include: Option<IncludeField>,
  pub group_by: Option<Vec<String>>,
}

impl Options {
  pub fn parse(&self) -> Result<QueryOptions, ParserError> {
    let limit = self.limit.map(|limit_str| limit_str.to_string());
    let offset = self.offset.map(|offset_str| offset_str.to_string());
    let num_of_rows = self.num_of_rows.map(|num_of_rows| num_of_rows as usize);
    let order_by = OrderBy::parse(self.order_by.clone());
    let include = self.include.clone().map(|include| include.parse());
    let group_by = self.group_by.clone();

    Ok(QueryOptions {
      order_by,
      limit,
      offset,
      num_of_rows,
      include,
      group_by,
    })
  }
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct Metadata {
  pub command: String,
  pub table: String,
  pub schema: String,
}

#[derive(Debug)]
#[napi(object)]
pub struct Statement {
  pub metadata: Metadata,
  pub fields: Vec<Field>,
  pub options: Options,
}

pub(crate) fn create_statement(metadata: Metadata, fields: Vec<Field>, options: Options) -> Result<ParsedStatement, ParserError> {
  let query = match create_query(metadata.clone(), fields, options) {
    Ok(query) => query,
    Err(err) => return Err(err),
  };

  let operation = match create_operation(metadata) {
    Ok(operation) => operation,
    Err(err) => return Err(err),
  };

  Ok(ParsedStatement { query, operation })
}

fn create_query(metadata: Metadata, fields: Vec<Field>, options: Options) -> Result<ParsedQuery, ParserError> {
  let options = match options.parse() {
    Ok(options) => options,
    Err(err) => return Err(err),
  };

  let table = &metadata.table;
  let schema = &metadata.schema;
  let fields = match fields.iter().map(|field| field.parse()).collect::<Result<Vec<_>, _>>() {
    Ok(fields) => fields,
    Err(err) => return Err(err),
  };

  Ok(ParsedQuery {
    table: table.to_string(),
    schema: schema.to_string(),
    fields,
    options,
  })
}

fn create_operation(metadata: Metadata) -> Result<ParsedOperation, ParserError> {
  match metadata.command.to_lowercase().as_str() {
    "findfirst" => Ok(ParsedOperation::FindFirst),
    "findmany" => Ok(ParsedOperation::FindMany),
    "updateone" => Ok(ParsedOperation::UpdateOne),
    "updatemany" => Ok(ParsedOperation::UpdateMany),
    "deleteone" => Ok(ParsedOperation::DeleteOne),
    "deletemany" => Ok(ParsedOperation::DeleteMany),
    "insertone" => Ok(ParsedOperation::InsertOne),
    "insertmany" => Ok(ParsedOperation::InsertMany),
    "count" => Ok(ParsedOperation::Count),
    "average" => Ok(ParsedOperation::Average),
    _ => Err(ParserError::InputError("Invalid operation!".to_string())),
  }
}
