pub fn camel_to_snake_case(s: &str) -> String {
  let mut snake_case = String::new();
  let mut chars = s.chars().peekable();

  while let Some(current_char) = chars.next() {
    if current_char.is_uppercase() {
      if let Some(next_char) = chars.peek() {
        if next_char.is_lowercase() {
          snake_case.push('_');
        }
      }
      snake_case.push(current_char.to_ascii_lowercase());
    } else {
      snake_case.push(current_char);
    }
  }

  snake_case
}

pub fn snake_to_camel_case(snake: &str) -> String {
  let mut camel = String::new();
  let mut capitalize_next = false;

  for c in snake.chars() {
    if c == '_' {
      capitalize_next = true;
    } else if capitalize_next {
      camel.push(c.to_ascii_uppercase());
      capitalize_next = false;
    } else {
      camel.push(c);
    }
  }

  camel
}
