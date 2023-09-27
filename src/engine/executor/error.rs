use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
  #[error("Invalid credentials: {0}")]
  InvalidCredentials(String),

  #[error("Error: {0}")]
  GenericError(String),

  #[error("Missing variable: {0}")]
  MissingVariable(String),

  #[error("RDS Data Service error: {0}")]
  Error(String),

  #[error(transparent)]
  NapiError(#[from] napi::Error<napi::Status>),
}

impl From<RuntimeError> for napi::Error<napi::Status> {
  fn from(runtime_error: RuntimeError) -> Self {
      // Convert your custom error to an appropriate status and reason for napi::Error.
      let status = match runtime_error {
          RuntimeError::InvalidCredentials(_) => napi::Status::InvalidArg,
          RuntimeError::GenericError(_) => napi::Status::GenericFailure,
          RuntimeError::MissingVariable(_) => napi::Status::GenericFailure,
          RuntimeError::Error(_) => napi::Status::GenericFailure,
          RuntimeError::NapiError(err) => return err,
      };

      let reason = runtime_error.to_string();

      napi::Error::new(status, reason)
  }
}