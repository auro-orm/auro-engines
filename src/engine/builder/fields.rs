use super::{arguments, error::BuilderError};
use crate::{
  constants::{constants, errors},
  parser::parsed::{ParsedField, ParsedInclude},
};

pub fn build_select(select_field: &ParsedField) -> Result<String, BuilderError> {
  let arguments_str = select_field
    .arguments
    .iter()
    .map(arguments::build_select)
    .collect::<Result<Vec<String>, BuilderError>>()?
    .join(", ");

  if arguments_str.is_empty() {
    return Err(BuilderError::InvalidStatement(errors::INVALID_STATEMENT));
  }

  Ok(format!("{} {} FROM", constants::SELECT, arguments_str))
}

pub fn build_where(where_field: &ParsedField) -> Result<String, BuilderError> {
  let arguments_str = where_field
    .arguments
    .iter()
    .map(arguments::build_where)
    .collect::<Result<Vec<String>, BuilderError>>()?
    .join(constants::AND);

  if arguments_str.is_empty() {
    return Err(BuilderError::InvalidStatement(errors::INVALID_STATEMENT));
  }

  Ok(format!("{} {}", constants::WHERE, arguments_str))
}

pub fn build_set(set_field: &ParsedField) -> Result<String, BuilderError> {
  let set_values = set_field
    .arguments
    .iter()
    .map(|arg| format!("{} = {}", arg.name, arguments::parse_value(&arg.value)))
    .collect::<Vec<_>>()
    .join(", ");

  if set_values.is_empty() {
    return Err(BuilderError::InvalidStatement(errors::INVALID_STATEMENT));
  }

  Ok(format!("SET {}", set_values))
}

pub fn build_one_data(data_field: &ParsedField) -> Result<String, BuilderError> {
  let arguments = &data_field.arguments;

  if arguments.is_empty() {
    return Err(BuilderError::InvalidStatement(errors::INVALID_STATEMENT));
  }

  let columns = arguments
    .iter()
    .map(|arg| arg.name.to_string())
    .collect::<Vec<_>>()
    .join(", ");

  let values = arguments
    .iter()
    .map(|arg| arguments::parse_value(&arg.value))
    .collect::<Vec<_>>()
    .join(", ");

  Ok(format!("({}) VALUES ({})", columns, values))
}

pub fn build_return(return_field: &ParsedField) -> Result<String, BuilderError> {
  let arguments = match &return_field
    .arguments
    .iter()
    .map(arguments::build_return)
    .collect::<Result<Vec<String>, BuilderError>>()
  {
    Ok(arguments) => arguments,
    Err(_err) => return Err(BuilderError::MissingArgumentValue(errors::INVALID_STATEMENT)),
  }
  .join(", ");

  Ok(format!("{}{}", constants::RETURN, arguments))
}

pub fn build_many_data(data_field: &ParsedField, num_rows: usize) -> Result<String, BuilderError> {
  let columns = data_field
    .arguments
    .iter()
    .map(|arg| arg.name.to_string())
    .collect::<Vec<_>>();

  let num_columns = columns.len();
  let num_columns_per_row = num_columns / num_rows;

  let mut columns = columns;
  columns.truncate(num_columns - (num_rows - 1) * num_columns_per_row);

  let num_rows = data_field.arguments.len() / num_columns_per_row;

  let mut values = Vec::new();
  let mut arg_index = 0; // Initialize the argument index.

  for _ in 0..num_rows {
    let row_args = &data_field.arguments[arg_index..arg_index + num_columns_per_row];

    arg_index += num_columns_per_row; // Update the argument index for the next row.

    let row_values = columns
      .iter()
      .map(|col| arguments::parse_value(&row_args.iter().find(|arg| &arg.name == col).unwrap().value))
      .collect::<Vec<_>>();

    let row_values_str = row_values.join(", ");
    values.push(format!("({})", row_values_str));
  }

  let columns_str = columns.join(", ");
  let values_str = values.join(",\n");

  Ok(format!("({})\nVALUES\n{}", columns_str, values_str))
}

pub fn build_include(parsed_include: &ParsedInclude, schema: &str) -> Result<String, BuilderError> {
  if parsed_include.joins.is_empty() {
    return Ok(String::new()); // No joins to include
  }

  let join_conditions: Vec<String> = parsed_include
    .joins
    .iter()
    .map(|(source_table, source_key, joining_table, joining_key)| {
      format!("{}.{} = {}.{}", source_table, source_key, joining_table, joining_key)
    })
    .collect();

  let join_condition_str = join_conditions.join(" AND ");
  let join_tables_str: Vec<String> = parsed_include
    .joins
    .iter()
    .map(|(_, _, table, _)| format!("{}.{}", schema, table))
    .collect();
  let join_tables_str = join_tables_str.join(", ");

  Ok(format!("INNER JOIN {} ON {}", join_tables_str, join_condition_str))
}
