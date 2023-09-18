use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuilderError {
  #[error("{:?}", _0)]
  InvalidFieldName(&'static str),

  #[error("{:?}", _0)]
  MissingSchema(&'static str),

  #[error("{:?}", _0)]
  MissingTable(&'static str),

  #[error("{:?}", _0)]
  MissingField(String),

  #[error("{:?}", _0)]
  InputError(&'static str),

  #[error("{:?}", _0)]
  InvalidStatement(&'static str),

  #[error("{:?}", _0)]
  MissingArgumentValue(&'static str),
}
