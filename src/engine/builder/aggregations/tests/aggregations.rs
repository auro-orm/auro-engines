#[cfg(test)]
mod tests {
  use crate::{
    engine::builder::aggregations::aggregations::{average, count},
    parser::parsed::{FieldName, ParsedArgument, ParsedField, ParsedQuery, ParsedValue, QueryOptions},
  };

  #[test]
  fn test_count() {
    let options = QueryOptions {
      order_by: None,
      limit: None,
      offset: None,
      num_of_rows: None,
      include: None,
      group_by: None,
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![ParsedField {
        name: FieldName::Where,
        arguments: vec![ParsedArgument {
          name: "arg1".to_string(),
          value: Some(ParsedValue::String("value1".to_string())),
        }],
      }],
    };

    let result = count(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(
      generated_sql,
      "SELECT COUNT (*) FROM my_schema.my_table WHERE arg1 = 'value1' "
    );
  }

  #[test]
  fn test_count_without_where() {
    let options = QueryOptions {
      order_by: None,
      limit: None,
      offset: None,
      num_of_rows: None,
      include: None,
      group_by: None,
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![],
    };

    let result = count(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(
      generated_sql,
      "SELECT COUNT (*) FROM my_schema.my_table  "
    );
  }

  #[test]
  fn test_average() {
    let options = QueryOptions {
      order_by: None,
      limit: None,
      offset: None,
      num_of_rows: None,
      include: None,
      group_by: None,
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![ParsedField {
        name: FieldName::Where,
        arguments: vec![ParsedArgument {
          name: "arg1".to_string(),
          value: Some(ParsedValue::String("value1".to_string())),
        }],
      }],
    };

    let result = average(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "SELECT AVG(id) FROM my_schema.my_table WHERE arg1 = 'value1' ");
  }

  #[test]
  fn test_average_without_where() {
    let options = QueryOptions {
      order_by: None,
      limit: None,
      offset: None,
      num_of_rows: None,
      include: None,
      group_by: None,
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![],
    };

    let result = average(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "SELECT AVG(id) FROM my_schema.my_table  ");
  }
}
