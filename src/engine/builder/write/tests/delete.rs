#[cfg(test)]
mod tests {
  use crate::{
    engine::builder::delete::{many, unique},
    parser::parsed::{FieldName, ParsedArgument, ParsedField, ParsedQuery, ParsedValue, QueryOptions},
  };

  #[test]
  fn test_unique() {
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
      fields: vec![
        ParsedField {
          name: FieldName::Return,
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

    let result = unique(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(
      generated_sql,
      "DELETE FROM my_schema.my_table WHERE arg3 = 'value3' RETURNING arg1 "
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
      options: options,
      fields: vec![
        ParsedField {
          name: FieldName::Where,
          arguments: vec![ParsedArgument {
            name: "arg3".to_string(),
            value: Some(ParsedValue::String("value3".to_string())),
          }],
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

    let result = many(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(
      generated_sql,
      "DELETE FROM my_schema.my_table WHERE arg3 = 'value3' RETURNING arg1 "
    );
  }

  #[test]
  fn test_unique_no_return() {
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
          name: FieldName::Where,
          arguments: vec![ParsedArgument {
            name: "arg3".to_string(),
            value: Some(ParsedValue::String("value3".to_string())),
          }],
        },
        ParsedField {
          name: FieldName::Return,
          arguments: vec![],
        },
      ],
    }; // Should return nothing

    let result = unique(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "DELETE FROM my_schema.my_table WHERE arg3 = 'value3'  ");
  }

  #[test]
  fn test_unique_with_options_and_return() {
    let options = QueryOptions {
      order_by: None,
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
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
          name: FieldName::Where,
          arguments: vec![ParsedArgument {
            name: "arg3".to_string(),
            value: Some(ParsedValue::String("value3".to_string())),
          }],
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

    let result = unique(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(
      generated_sql,
      "DELETE FROM my_schema.my_table WHERE arg3 = 'value3' RETURNING arg1  LIMIT 10 OFFSET 5"
    );
  }

  #[test]
  fn test_unique_with_options_and_no_return() {
    let options = QueryOptions {
      order_by: Some(Default::default()),
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: None,
      include: None,
      group_by: None,
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options.clone(),
      fields: vec![ParsedField {
        name: FieldName::Where,
        arguments: vec![ParsedArgument {
          name: "arg3".to_string(),
          value: Some(ParsedValue::String("value3".to_string())),
        }],
      }],
    };

    let result = unique(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(
      generated_sql,
      "DELETE FROM my_schema.my_table WHERE arg3 = 'value3' RETURNING * ORDER BY  LIMIT 10 OFFSET 5"
    );
  }
}
