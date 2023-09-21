#[cfg(test)]
pub mod tests {
  use parser::{
    parsed::{
      FieldName, ParsedArgument, ParsedField, ParsedInclude, ParsedOperation, ParsedQuery, ParsedStatement, ParsedValue,
      QueryOptions,
    },
    statement::{create_statement, Argument, Field, IncludeField, Join, Metadata, Options, Order, OrderBy},
  };
  use std::{collections::HashMap, str::FromStr};

  use crate::parser;

  #[test]
  fn test_include_field_parse() {
    let include_field = IncludeField {
      joins: vec![Join {
        table: "Table1".to_string(),
        key: "Key1".to_string(),
        joining_table: "Table2".to_string(),
        joining_key: "Key2".to_string(),
      }],
    };

    let parsed_include = include_field.parse();

    let expected_parsed_include = ParsedInclude {
      joins: vec![(
        "Table1".to_string(),
        "Key1".to_string(),
        "Table2".to_string(),
        "Key2".to_string(),
      )],
    };

    assert_eq!(parsed_include, expected_parsed_include);
  }

  // Test Field::parse function
  #[test]
  fn test_field_parse() {
    let field = Field {
      name: "select".to_string(),
      arguments: vec![Argument {
        name: "arg1".to_string(),
        value: Some("value1".to_string()),
        value_type: Some("string".to_string()),
      }],
    };

    let parsed_field_result = field.parse();

    let expected_parsed_field = Ok(ParsedField {
      name: FieldName::Select,
      arguments: vec![ParsedArgument {
        name: "arg1".to_string(),
        value: Some(ParsedValue::String("value1".to_string())),
      }],
    });

    assert_eq!(parsed_field_result, expected_parsed_field);
  }

  // Test FieldName::from_str function
  #[test]
  fn test_field_name_from_str() {
    assert_eq!(FieldName::from_str("select"), Ok(FieldName::Select));
  }

  // Test Argument::parse function
  #[test]
  fn test_argument_parse() {
    let argument = Argument {
      name: "arg1".to_string(),
      value: Some("value1".to_string()),
      value_type: Some("string".to_string()),
    };

    let parsed_argument_result = argument.parse();

    let expected_parsed_argument = Ok(ParsedArgument {
      name: "arg1".to_string(),
      value: Some(ParsedValue::String("value1".to_string())),
    });

    assert_eq!(parsed_argument_result, expected_parsed_argument);
  }

  // Test OrderBy::parse function
  #[test]
  fn test_order_by_parse() {
    let order_by = OrderBy {
      field: "field1".to_string(),
      order: Order::Asc,
    };

    let order_by_vec = vec![order_by];
    let parsed_order_by = OrderBy::parse(Some(order_by_vec));

    let mut expected_order_by_map = HashMap::new();
    expected_order_by_map.insert("field1".to_string(), "asc".to_string());

    assert_eq!(parsed_order_by, Some(expected_order_by_map));
  }

  // Test Order::parse function
  #[test]
  fn test_order_parse() {
    assert_eq!(Order::Asc.parse(), "asc");
    assert_eq!(Order::Desc.parse(), "desc");
  }

  // Test Options::parse function
  #[test]
  fn test_options_parse() {
    let options = Options {
      order_by: Some(vec![OrderBy {
        field: "field1".to_string(),
        order: Order::Asc,
      }]),
      limit: Some(10),
      offset: Some(5),
      num_of_rows: Some(20),
      include: None,
      group_by: Some(vec!["group1".to_string(), "group2".to_string()]),
    };

    let parsed_options_result = options.parse();

    let expected_parsed_options = Ok(QueryOptions {
      order_by: Some(HashMap::from_iter(vec![("field1".to_string(), "asc".to_string())])),
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: Some(20),
      include: None,
      group_by: Some(vec!["group1".to_string(), "group2".to_string()]),
    });

    assert_eq!(parsed_options_result, expected_parsed_options);
  }

  // Test create_statement function
  #[test]
  fn test_create_statement() {
    let metadata = Metadata {
      command: "findfirst".to_string(),
      table: "table1".to_string(),
      schema: "schema1".to_string(),
    };

    let fields = vec![Field {
      name: "select".to_string(),
      arguments: vec![Argument {
        name: "arg1".to_string(),
        value: Some("value1".to_string()),
        value_type: Some("string".to_string()),
      }],
    }];

    let options = Options {
      order_by: Some(vec![OrderBy {
        field: "field1".to_string(),
        order: Order::Asc,
      }]),
      limit: Some(10),
      offset: Some(5),
      num_of_rows: Some(20),
      include: None,
      group_by: Some(vec!["group1".to_string(), "group2".to_string()]),
    };

    let parsed_statement_result = create_statement(metadata, fields, options);

    let expected_parsed_statement = Ok(ParsedStatement {
      query: ParsedQuery {
        table: "table1".to_string(),
        schema: "schema1".to_string(),
        fields: vec![ParsedField {
          name: FieldName::Select,
          arguments: vec![ParsedArgument {
            name: "arg1".to_string(),
            value: Some(ParsedValue::String("value1".to_string())),
          }],
        }],
        options: QueryOptions {
          order_by: Some(HashMap::from_iter(vec![("field1".to_string(), "asc".to_string())])),
          limit: Some("10".to_string()),
          offset: Some("5".to_string()),
          num_of_rows: Some(20),
          include: None,
          group_by: Some(vec!["group1".to_string(), "group2".to_string()]),
        },
      },
      operation: ParsedOperation::FindFirst,
    });

    assert_eq!(parsed_statement_result, expected_parsed_statement);
  }
}
