#[cfg(test)]
mod update_tests {
  use crate::{
    engine::builder::update::{many, one},
    parser::parsed::{FieldName, ParsedArgument, ParsedField, ParsedQuery, ParsedValue, QueryOptions},
  };

  #[test]
  fn test_one() {
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
      options: options.clone(),
      fields: vec![
        ParsedField {
          name: FieldName::Set,
          arguments: vec![ParsedArgument {
            name: "arg1".to_string(),
            value: Some(ParsedValue::String("value1".to_string())),
          }],
        },
        ParsedField {
          name: FieldName::Where,
          arguments: vec![ParsedArgument {
            name: "arg3".to_string(),
            value: Some(ParsedValue::String("value3".to_string())),
          }],
        },
      ],
    };

    let result = one(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(
      generated_sql,
      "UPDATE my_schema.my_table SET arg1 = 'value1' WHERE arg3 = 'value3' RETURNING * "
    );
  }

  #[test]
  fn test_many() {
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
      options: options.clone(),
      fields: vec![
        ParsedField {
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
        }
      ],
    };

    let result = many(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(
      generated_sql,
      "UPDATE my_schema.my_table SET arg1 = 'value1', arg2 = 'value2'  RETURNING * "
    );
  }

  #[test]
  fn test_one_with_return() {
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
      options: options.clone(),
      fields: vec![
        ParsedField {
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
        },
        ParsedField {
          name: FieldName::Return,
          arguments: vec![ParsedArgument {
            name: "arg1".to_string(),
            value: Some(ParsedValue::String("value1".to_string())),
          }],
        },
      ],
    };

    let result = one(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(
      generated_sql,
      "UPDATE my_schema.my_table SET arg1 = 'value1', arg2 = 'value2'  RETURNING arg1 "
    );
  }

  #[test]
  fn test_one_with_no_return() {
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
      options: options.clone(),
      fields: vec![
        ParsedField {
          name: FieldName::Set,
          arguments: vec![ParsedArgument {
            name: "arg1".to_string(),
            value: Some(ParsedValue::String("value1".to_string())),
          }],
        },
        ParsedField {
          name: FieldName::NoReturn,
          arguments: vec![],
        },
      ],
    };

    let result = one(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "UPDATE my_schema.my_table SET arg1 = 'value1'   ");
  }
}
