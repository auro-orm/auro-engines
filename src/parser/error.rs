use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum ParserError {
  #[error("{:?}", _0)]
  SchemaError(String),

  #[error("{:?}", _0)]
  ParseError(String),

  #[error("{:?}", _0)]
  InputError(String),

  #[error("{:?}", _0)]
  NotFoundError(String),

  #[error("{:?}", _0)]
  Error(String),

  #[error("{:?}", _0)]
  NoArguments(String),
}
