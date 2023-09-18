use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
  #[error("{:?}", _0)]
  InvalidCredentials(String),

  #[error("{:?}", _0)]
  Error(String),

  #[error("{:?}", _0)]
  MissingVariable(String),
}