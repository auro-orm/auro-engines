#[cfg(test)]
mod option_tests {
  use std::collections::HashMap;

  use crate::{engine::builder::options::build_options, parser::parsed::QueryOptions};

  #[test]
  fn test_build_options() {
    let options = QueryOptions {
      order_by: Some({
        let mut order_by = HashMap::new();
        order_by.insert("fieldName".to_string(), "asc".to_string());
        order_by
      }),
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: Some(100),
      include: None, // You can provide a ParsedInclude here
      group_by: Some(vec!["column1".to_string(), "column2".to_string()]),
    };

    let result = build_options(options);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(
      generated_sql,
      "ORDER BY field_name ASC LIMIT 10 OFFSET 5 GROUP BY column1, column2"
    );
  }

  #[test]
  fn test_build_options_with_empty_group_by() {
    let options = QueryOptions {
      order_by: Some({
        let mut order_by = HashMap::new();
        order_by.insert("fieldName".to_string(), "asc".to_string());
        order_by
      }),
      limit: Some("10".to_string()),
      offset: Some("5".to_string()),
      num_of_rows: Some(100),
      include: None, // You can provide a ParsedInclude here
      group_by: Some(vec![]),
    };

    let result = build_options(options);
    assert!(result.is_ok());

    let generated_sql = result.unwrap();
    assert_eq!(generated_sql, "ORDER BY field_name ASC LIMIT 10 OFFSET 5");
  }
}
