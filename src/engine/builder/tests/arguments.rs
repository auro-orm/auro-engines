#[cfg(test)]
mod args_tests {
  use crate::{parser::parsed::{ParsedValue, ParsedArgument}, engine::builder::arguments::{build_where, build_return, build_select, parse_value}};

  #[test]
  fn test_build_where_with_custom_value() {
    let argument = ParsedArgument {
      name: "column_name".to_string(),
      value: Some(ParsedValue::Custom("custom_value".to_string())),
    };
    let result = build_where(&argument);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "column_name custom_value");
  }

  #[test]
  fn test_build_where_with_other_values() {
    // Test with ParsedValue::String
    let argument_string = ParsedArgument {
      name: "column_name".to_string(),
      value: Some(ParsedValue::String("value1".to_string())),
    };
    let result_string = build_where(&argument_string);
    assert!(result_string.is_ok());
    let generated_sql_string = result_string.unwrap();
    assert_eq!(generated_sql_string, "column_name = 'value1'");

    // Test with ParsedValue::Number
    let argument_number = ParsedArgument {
      name: "column_name".to_string(),
      value: Some(ParsedValue::Number("123".to_string())),
    };
    let result_number = build_where(&argument_number);
    assert!(result_number.is_ok());
    let generated_sql_number = result_number.unwrap();
    assert_eq!(generated_sql_number, "column_name = 123");

    // Test with ParsedValue::Null
    let argument_null = ParsedArgument {
      name: "column_name".to_string(),
      value: Some(ParsedValue::Null),
    };
    let result_null = build_where(&argument_null);
    assert!(result_null.is_ok());
    let generated_sql_null = result_null.unwrap();
    assert_eq!(generated_sql_null, "column_name = NULL");
  }

  #[test]
  fn test_build_return() {
    let argument = ParsedArgument {
      name: "column_name".to_string(),
      value: Some(ParsedValue::String("value1".to_string())),
    };
    let result = build_return(&argument);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "column_name");
  }

  #[test]
  fn test_build_select() {
    let argument = ParsedArgument {
      name: "column_name".to_string(),
      value: Some(ParsedValue::String("value1".to_string())),
    };
    let result = build_select(&argument);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "column_name");
  }

  #[test]
  fn test_parse_value() {
    // Test with ParsedValue::String
    let parsed_value_string = Some(ParsedValue::String("value1".to_string()));
    let result_string = parse_value(&parsed_value_string);
    assert_eq!(result_string, "'value1'");

    // Test with ParsedValue::Number
    let parsed_value_number = Some(ParsedValue::Number("123".to_string()));
    let result_number = parse_value(&parsed_value_number);
    assert_eq!(result_number, "123");

    // Test with ParsedValue::Null
    let parsed_value_null = Some(ParsedValue::Null);
    let result_null = parse_value(&parsed_value_null);
    assert_eq!(result_null, "NULL");
  }
}
