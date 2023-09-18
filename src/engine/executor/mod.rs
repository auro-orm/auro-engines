pub mod error;
pub mod utils;
use self::error::RuntimeError;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_rdsdata::{
  client::fluent_builders::ExecuteStatement, model::RecordsFormatType, output::ExecuteStatementOutput, types::SdkError, Client,
  Region,
};
use dotenv::dotenv;
use napi_derive::napi;

#[napi(object)]
#[derive(Debug, Clone)]
pub struct ConnectionOptions {
  pub resource_arn: String,
  pub secret_arn: String,
  pub database: String,
  pub region: String,
}

impl ConnectionOptions {
  pub fn new(resource_arn: String, secret_arn: String, database: String, region: String) -> Self {
    Self {
      resource_arn,
      secret_arn,
      database,
      region,
    }
  }
}

pub async fn get_foreign_keys(client: &ExecuteStatement, table: &str) -> Result<String, RuntimeError> {
  dotenv().ok();
  let schema = std::env::var("REGION").expect("Schema must be set.");

  let introspection_query = format!(
    "SELECT
    tc.table_schema, 
    tc.constraint_name, 
    tc.table_name, 
    kcu.column_name, 
    ccu.table_schema AS foreign_table_schema,
    ccu.table_name AS foreign_table_name,
    ccu.column_name AS foreign_column_name 
FROM information_schema.table_constraints AS tc 
JOIN information_schema.key_column_usage AS kcu
    ON tc.constraint_name = kcu.constraint_name
    AND tc.table_schema = kcu.table_schema
JOIN information_schema.constraint_column_usage AS ccu
    ON ccu.constraint_name = tc.constraint_name
WHERE tc.constraint_type = 'FOREIGN KEY'
    AND tc.table_schema='{}'
    AND tc.table_name='{}';
  ",
    schema, table
  );

  let st = client.clone().sql(introspection_query);

  match st.send().await {
    Ok(result) => {
      let formatted_records = result.formatted_records().unwrap();
      Ok(formatted_records.to_string())
    }
    Err(error) => {
      if let SdkError::ServiceError(service_error) = error {
        Err(RuntimeError::Error(format!("{}", service_error.err())))
      } else {
        Err(RuntimeError::Error("Unknown error occurred".to_string()))
      }
    }
  }
}

pub async fn introspect_schema(client: &ExecuteStatement) -> Result<String, RuntimeError> {
  dotenv().ok();
  let schema = std::env::var("REGION").expect("Schema must be set.");

  let introspection_query = format!(
    "SELECT tabl.table_name, columns.column_name, columns.data_type, columns.is_nullable,
      CASE
        WHEN primary_keys.column_name IS NOT NULL THEN 'YES'
        ELSE 'NO'
      END AS is_primary_key
    FROM
      (SELECT table_name FROM information_schema.tables WHERE table_schema = '{}') AS tabl
    INNER JOIN 
      (SELECT
        table_name,
        column_name,
        data_type,
        is_nullable
      FROM information_schema.columns) AS columns
    ON tabl.table_name = columns.table_name
    LEFT JOIN (
      SELECT tc.table_name, kcu.column_name
      FROM information_schema.table_constraints tc
      JOIN information_schema.key_column_usage kcu
        ON tc.constraint_name = kcu.constraint_name
      AND tc.table_schema = kcu.table_schema
      AND tc.table_name = kcu.table_name
      WHERE tc.constraint_type = 'PRIMARY KEY'
    ) AS primary_keys
    ON tabl.table_name = primary_keys.table_name
      AND columns.column_name = primary_keys.column_name
    GROUP BY
      tabl.table_name,
      columns.column_name,
      columns.data_type,
      columns.is_nullable,
      is_primary_key;
  ",
    schema
  );

  let st = client.clone().sql(introspection_query);

  match st.send().await {
    Ok(result) => {
      let formatted_records = result.formatted_records().unwrap();
      Ok(formatted_records.to_string())
    }
    Err(error) => {
      if let SdkError::ServiceError(service_error) = error {
        Err(RuntimeError::Error(format!("{}", service_error.err())))
      } else {
        Err(RuntimeError::Error("Unknown error occurred".to_string()))
      }
    }
  }
}

fn get_connection_options() -> ConnectionOptions {
  dotenv().ok();
  let resource_arn = std::env::var("RESOURCE_ARN").expect("RESOURCE_ARN must be set.");
  let secret_arn = std::env::var("SECRET_ARN").expect("SECRET_ARN must be set.");
  let database = std::env::var("DATABASE").expect("DATABASE must be set.");
  let region = std::env::var("REGION").expect("REGION must be set.");

  return ConnectionOptions::new(resource_arn, secret_arn, database, region);
}

pub async fn execute_statement(query: String, client: &ExecuteStatement) -> Result<ExecuteStatementOutput, RuntimeError> {
  let st = client.clone().sql(query);

  match st.send().await {
    Ok(result) => Ok(result),
    Err(error) => {
      if let SdkError::ServiceError(service_error) = error {
        Err(RuntimeError::Error(format!("{}", service_error.err())))
      } else {
        Err(RuntimeError::Error("Unknown error occurred".to_string()))
      }
    }
  }
}

pub async fn initialize_client() -> ExecuteStatement {
  let options = get_connection_options();
  let region = Region::new(options.region);

  let region_provider = RegionProviderChain::first_try(region.clone())
    .or_default_provider()
    .or_else(Region::new("eu-central-1"));

  let shared_config = aws_config::from_env().region(region_provider).load().await;

  Client::new(&shared_config)
    .execute_statement()
    .resource_arn(options.resource_arn)
    .database(options.database)
    .secret_arn(options.secret_arn)
    .format_records_as(RecordsFormatType::Json)
}
