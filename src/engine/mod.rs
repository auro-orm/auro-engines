use self::executor::error::RuntimeError;
use crate::{
  engine::{builder::build, executor::execute_statement},
  parser::{self, parsed},
};
use aws_sdk_rdsdata::client::fluent_builders::ExecuteStatement;

pub mod builder;
pub mod executor;

pub(crate) async fn execute(statement: &parsed::ParsedStatement, client: &ExecuteStatement) -> Result<Option<String>, RuntimeError> {
  let query_string = match build(&statement) {
    Ok(query_string) => query_string,
    Err(err) => return Err(RuntimeError::Error(err.to_string())),
  };

  let result = match execute_statement(query_string, client).await {
    Ok(result) => result,
    Err(err) => return Err(RuntimeError::Error(err.to_string())),
  };

  let formatted_records = match result.formatted_records() {
    Some(formatted_records) => Some(parser::utils::snake_to_camel_case(formatted_records)),
    None => None,
  };

  Ok(formatted_records)
}
