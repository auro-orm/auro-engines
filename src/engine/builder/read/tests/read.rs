#[cfg(test)]
mod tests {
  use crate::{
    engine::builder::read::find::{many, unique},
    parser::parsed::{FieldName, ParsedArgument, ParsedField, ParsedQuery, ParsedValue, QueryOptions, ParsedInclude},
  };

  #[test]
  fn test_unique() {
    let options = QueryOptions {
      order_by: None,
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: Some(100),
      include: None,
      group_by: Some(vec![]),
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![
        ParsedField {
          name: FieldName::Select,
          arguments: vec![ParsedArgument {
            name: "arg1".to_string(),
            value: Some(ParsedValue::String("value1".to_string())),
          }],
        },
        ParsedField {
          name: FieldName::Where,
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
      "SELECT arg1 FROM my_schema.my_table  WHERE arg1 = 'value1'  LIMIT 10 OFFSET 5"
    );
  }

  #[test]
  fn test_many() {
    let options = QueryOptions {
      order_by: None,
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: Some(100),
      include: None,
      group_by: Some(vec![]),
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![
        ParsedField {
          name: FieldName::Select,
          arguments: vec![ParsedArgument {
            name: "arg1".to_string(),
            value: Some(ParsedValue::String("value1".to_string())),
          }],
        },
        ParsedField {
          name: FieldName::Where,
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
      "SELECT arg1 FROM my_schema.my_table WHERE arg1 = 'value1'  LIMIT 10 OFFSET 5"
    );
  }

  #[test]
  fn test_unique_with_order_by() {
    let options = QueryOptions {
      order_by: Some(vec![("field1".to_string(), "asc".to_string())].into_iter().collect()),
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: Some(100),
      include: None,
      group_by: Some(vec![]),
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![
        ParsedField {
          name: FieldName::Select,
          arguments: vec![ParsedArgument {
            name: "arg1".to_string(),
            value: Some(ParsedValue::String("value1".to_string())),
          }],
        },
        ParsedField {
          name: FieldName::Where,
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
      "SELECT arg1 FROM my_schema.my_table  WHERE arg1 = 'value1' ORDER BY field1 ASC LIMIT 10 OFFSET 5"
    );
  }

  #[test]
  fn test_many_with_include() {
    let options = QueryOptions {
      order_by: None,
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: Some(100),
      include: None,
      group_by: Some(vec![]),
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![
        ParsedField {
          name: FieldName::Select,
          arguments: vec![ParsedArgument {
            name: "arg1".to_string(),
            value: Some(ParsedValue::String("value1".to_string())),
          }],
        },
        ParsedField {
          name: FieldName::Where,
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
      "SELECT arg1 FROM my_schema.my_table WHERE arg1 = 'value1'  LIMIT 10 OFFSET 5"
    );
  }

  #[test]
  fn test_unique_with_group_by() {
    let options = QueryOptions {
      order_by: None,
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: Some(100),
      include: None,
      group_by: Some(vec!["column1".to_string(), "column2".to_string()]),
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![
        ParsedField {
          name: FieldName::Select,
          arguments: vec![ParsedArgument {
            name: "arg1".to_string(),
            value: Some(ParsedValue::String("value1".to_string())),
          }],
        },
        ParsedField {
          name: FieldName::Where,
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
      "SELECT arg1 FROM my_schema.my_table  WHERE arg1 = 'value1'  LIMIT 10 OFFSET 5 GROUP BY column1, column2"
    );
  }

  #[test]
  fn test_many_with_empty_group_by() {
    let options = QueryOptions {
      order_by: None,
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: Some(100),
      include: None,
      group_by: Some(vec![]),
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![
        ParsedField {
          name: FieldName::Select,
          arguments: vec![ParsedArgument {
            name: "arg1".to_string(),
            value: Some(ParsedValue::String("value1".to_string())),
          }],
        },
        ParsedField {
          name: FieldName::Where,
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
      "SELECT arg1 FROM my_schema.my_table WHERE arg1 = 'value1'  LIMIT 10 OFFSET 5"
    );
  }

  #[test]
  fn test_unique_with_no_options() {
    let options = QueryOptions {
      order_by: None,
      limit: None,
      offset: None,
      num_of_rows: Some(100),
      include: None,
      group_by: None,
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![
        ParsedField {
          name: FieldName::Select,
          arguments: vec![ParsedArgument {
            name: "arg1".to_string(),
            value: Some(ParsedValue::String("value1".to_string())),
          }],
        },
        ParsedField {
          name: FieldName::Where,
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
    assert_eq!(generated_sql, "SELECT arg1 FROM my_schema.my_table  WHERE arg1 = 'value1' ");
  }

  #[test]
  fn test_many_without_select_but_with_where() {
    let options = QueryOptions {
      order_by: None,
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: Some(100),
      include: None,
      group_by: Some(vec![]),
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![
        ParsedField {
          name: FieldName::Where,
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
      "SELECT * FROM my_schema.my_table WHERE arg1 = 'value1'  LIMIT 10 OFFSET 5"
    );
  }

  #[test]
  fn test_unique_without_select_and_where() {
    let options = QueryOptions {
      order_by: None,
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: Some(100),
      include: None,
      group_by: Some(vec![]),
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![],
    };

    let result = unique(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "SELECT * FROM my_schema.my_table    LIMIT 10 OFFSET 5");
  }

  #[test]
  fn test_unique_with_include() {
    let include_field = ParsedInclude {
        joins: vec![("table1".to_string(), "column1".to_string(), "table2".to_string(), "column2".to_string())]
    };

    let options = QueryOptions {
      order_by: None,
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: Some(100),
      include: Some(include_field),
      group_by: Some(vec![]),
    };

    let parsed_query = ParsedQuery {
      table: "my_table".to_string(),
      schema: "my_schema".to_string(),
      options: options,
      fields: vec![],
    };

    let result = unique(&parsed_query);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "SELECT * FROM my_schema.my_table INNER JOIN my_schema.table2 ON table1.column1 = table2.column2   LIMIT 10 OFFSET 5");
  }
}
