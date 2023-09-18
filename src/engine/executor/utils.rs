
pub fn format_error_message(error_message: &str) -> String {
    error_message
      .replace("\\n", ";")
      .replace("\\", "") // Remove backslashes
      .replace('"', "") // Remove double quotes
      .trim() // Trim leading and trailing spaces
      .to_string() // Convert to String
  }