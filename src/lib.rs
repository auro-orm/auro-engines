#![deny(clippy::all)]
use aws_sdk_rdsdata::client::fluent_builders::ExecuteStatement;
use engine::{
  execute,
  executor::{execute_statement, initialize_client, introspect_schema, utils::format_error_message, get_foreign_keys},
};
use lazy_static::lazy_static;
use napi_derive::napi;
use parser::statement::{create_statement, Field, Metadata, Options};
use tokio::sync::OnceCell;

pub mod constants;
pub mod engine;
pub mod parser;

lazy_static! {
  static ref CLIENT: OnceCell<ExecuteStatement> = OnceCell::new();
}

#[napi]
pub async fn connect() {
  let _client = CLIENT.get_or_init(initialize_client).await;
}

#[napi]
pub async fn introspect() -> napi::Result<String> {
  let client = CLIENT.get_or_init(initialize_client).await;

  let result = match introspect_schema(client).await {
    Ok(result) => result,
    Err(err) => return Err(napi::Error::from_reason(err.to_string())),
  };

  Ok(result)
}

#[napi]
pub async fn get_foreign_keys_data(table: String) -> napi::Result<String> {
  let client = CLIENT.get_or_init(initialize_client).await;

  let result = match get_foreign_keys(client, &table).await {
    Ok(result) => result,
    Err(err) => return Err(napi::Error::from_reason(err.to_string())),
  };

  Ok(result)
}

#[napi]
pub async fn query(fields: Vec<Field>, options: Options, metadata: Metadata) -> napi::Result<Option<String>> {
  let client = CLIENT.get_or_init(initialize_client).await;

  let statement = match create_statement(metadata, fields, options) {
    Ok(statement) => statement,
    Err(err) => return Err(napi::Error::from(err)),
  };

  let formatted_records = match execute(&statement, client).await {
    Ok(formatted_records) => formatted_records,
    Err(err) => return Err(napi::Error::from_reason(format_error_message(&err.to_string()))),
  };

  Ok(formatted_records)
}

#[napi]
pub async fn query_raw(query_string: String) -> napi::Result<Option<String>> {
  let client = CLIENT.get_or_init(initialize_client).await;

  let result = match execute_statement(query_string, &client).await {
    Ok(result) => result,
    Err(err) => return Err(napi::Error::from_reason(format_error_message(&err.to_string()))),
  };

  let formatted_records = match result.formatted_records() {
    Some(formatted_records) => Some(parser::utils::snake_to_camel_case(formatted_records)),
    None => None,
  };

  Ok(formatted_records)
}
