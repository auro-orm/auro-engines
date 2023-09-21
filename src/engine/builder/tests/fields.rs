#[cfg(test)]
mod filed_tests {
  use crate::{
    constants::errors,
    engine::builder::{
      error::BuilderError,
      fields::{build_one_data, build_return, build_select, build_set, build_where},
    },
    parser::parsed::{FieldName, ParsedArgument, ParsedField, ParsedValue},
  };

  #[test]
  fn test_build_select() {
    let parsed_field = ParsedField {
      name: FieldName::Select,
      arguments: vec![
        ParsedArgument {
          name: "arg1".to_string(),
          value: Some(ParsedValue::String("value1".to_string())),
        },
        ParsedArgument {
          name: "arg2".to_string(),
          value: Some(ParsedValue::String("value2".to_string())),
        },
      ],
    };
    let result = build_select(&parsed_field);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "SELECT arg1, arg2 FROM");

    // Test with no arguments
    let parsed_field_no_args = ParsedField {
      name: FieldName::Select,
      arguments: vec![],
    };
    let result_no_args = build_select(&parsed_field_no_args);
    assert!(result_no_args.is_err());
    assert_eq!(
      result_no_args.err().unwrap(),
      BuilderError::InvalidStatement(errors::INVALID_STATEMENT)
    );
  }

  #[test]
  fn test_build_where() {
    let parsed_field = ParsedField {
      name: FieldName::Where,
      arguments: vec![
        ParsedArgument {
          name: "arg1".to_string(),
          value: Some(ParsedValue::String("value1".to_string())),
        },
        ParsedArgument {
          name: "arg2".to_string(),
          value: Some(ParsedValue::String("value2".to_string())),
        },
      ],
    };
    let result = build_where(&parsed_field);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "WHERE arg1 = 'value1' AND arg2 = 'value2'");

    // Test with no arguments
    let parsed_field_no_args = ParsedField {
      name: FieldName::Where,
      arguments: vec![],
    };
    let result_no_args = build_where(&parsed_field_no_args);
    assert!(result_no_args.is_err());
    assert_eq!(
      result_no_args.err().unwrap(),
      BuilderError::InvalidStatement(errors::INVALID_STATEMENT)
    );
  }

  #[test]
  fn test_build_set() {
    let parsed_field = ParsedField {
      name: FieldName::Set,
      arguments: vec![
        ParsedArgument {
          name: "arg1".to_string(),
          value: Some(ParsedValue::String("value1".to_string())),
        },
        ParsedArgument {
          name: "arg2".to_string(),
          value: Some(ParsedValue::String("value2".to_string())),
        },
      ],
    };
    let result = build_set(&parsed_field);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "SET arg1 = 'value1', arg2 = 'value2'");

    // Test with no arguments
    let parsed_field_no_args = ParsedField {
      name: FieldName::Set,
      arguments: vec![],
    };
    let result_no_args = build_set(&parsed_field_no_args);
    assert!(result_no_args.is_err());
    assert_eq!(
      result_no_args.err().unwrap(),
      BuilderError::InvalidStatement(errors::INVALID_STATEMENT)
    );
  }

  #[test]
  fn test_build_one_data() {
    let parsed_field = ParsedField {
      name: FieldName::Data,
      arguments: vec![
        ParsedArgument {
          name: "arg1".to_string(),
          value: Some(ParsedValue::String("value1".to_string())),
        },
        ParsedArgument {
          name: "arg2".to_string(),
          value: Some(ParsedValue::String("value2".to_string())),
        },
      ],
    };
    let result = build_one_data(&parsed_field);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "(arg1, arg2) VALUES ('value1', 'value2')");

    // Test with no arguments
    let parsed_field_no_args = ParsedField {
      name: FieldName::Data,
      arguments: vec![],
    };
    let result_no_args = build_one_data(&parsed_field_no_args);
    assert!(result_no_args.is_err());
    assert_eq!(
      result_no_args.err().unwrap(),
      BuilderError::InvalidStatement(errors::INVALID_STATEMENT)
    );
  }

  #[test]
  fn test_build_return() {
    let parsed_field = ParsedField {
      name: FieldName::Return,
      arguments: vec![
        ParsedArgument {
          name: "arg1".to_string(),
          value: Some(ParsedValue::String("value1".to_string())),
        },
        ParsedArgument {
          name: "arg2".to_string(),
          value: Some(ParsedValue::String("value2".to_string())),
        },
      ],
    };
    let result = build_return(&parsed_field);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "RETURNING arg1, arg2");

    // Test with no arguments
    let parsed_field_no_args = ParsedField {
      name: FieldName::Return,
      arguments: vec![],
    };
    let result_no_args = build_return(&parsed_field_no_args);
    assert!(result_no_args.is_ok());
    assert_eq!(result_no_args.unwrap(), String::from(""));
  }
}
