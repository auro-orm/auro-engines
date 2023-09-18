use super::error::BuilderError;
use crate::parser::parsed::{ParsedArgument, ParsedValue};

pub(crate) fn build_where(argument: &ParsedArgument) -> Result<String, BuilderError> {
  let value_string = match &argument.value {
    Some(ParsedValue::Custom(custom_value)) => {
      format!("{} {}", argument.name, custom_value)
    }
    _ => {
      format!("{} = {}", argument.name, parse_value(&argument.value))
    }
  };

  Ok(value_string)
}

pub(crate) fn build_return(argument: &ParsedArgument) -> Result<String, BuilderError> {
  Ok(argument.name.to_string())
}

pub(crate) fn build_select(argument: &ParsedArgument) -> Result<String, BuilderError> {
  Ok(argument.name.to_string())
}

pub(crate) fn parse_value(value: &Option<ParsedValue>) -> String {
  match value {
    Some(ParsedValue::String(s)) | Some(ParsedValue::Date(s)) | Some(ParsedValue::DateTime(s)) => format!("'{}'", s),
    Some(ParsedValue::Number(n)) => n.to_string(),
    Some(ParsedValue::Boolean(b)) => b.to_string(),
    Some(ParsedValue::Float(f)) => f.to_string(),
    Some(ParsedValue::Null) => "NULL".to_string(),
    Some(ParsedValue::Custom(c)) => c.to_string(),
    None => "NULL".to_string(),
  }
}
