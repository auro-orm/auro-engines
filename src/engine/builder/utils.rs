pub fn build_string_from_parts(parts: Vec<&str>) -> String {
  let mut result = String::new();
  for part in parts {
    result.push_str(&part);
  }
  result
}