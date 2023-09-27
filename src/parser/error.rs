use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParserError {
  #[error("Schema error: {0}")]
  SchemaError(String),

  #[error("Parse error: {0}")]
  ParseError(String),

  #[error("Input error: {0}")]
  InputError(String),

  #[error("Not found error: {0}")]
  NotFoundError(String),

  #[error("General error: {0}")]
  Error(String),

  #[error("No arguments error: {0}")]
  NoArguments(String),

  #[error("Invalid field name: {0}")]
  InvalidFieldName(String),

  #[error("Invalid value type: {0}")]
  InvalidValueType(String),

  #[error("Invalid order: {0}")]
  InvalidOrder(String),

  #[error("Invalid operation: {0}")]
  InvalidOperation(String),
}
