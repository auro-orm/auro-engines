use std::collections::HashMap;

use super::{error::BuilderError, utils};
use crate::{
  constants::constants,
  parser::{parsed::QueryOptions, utils::camel_to_snake_case},
};

fn build_order_by(order_by: &HashMap<String, String>) -> Result<String, BuilderError> {
  let mut order_by_parts = Vec::new();
  for (field, order) in order_by.iter() {
    let mut field_parts = Vec::new();
    field_parts.push(camel_to_snake_case(field.as_str()));
    field_parts.push(order.to_string().to_uppercase());
    order_by_parts.push(field_parts.join(" "));
  }

  let order_by_option = utils::build_string_from_parts(vec![constants::ORDER_BY, order_by_parts.join(",").as_str()]);
  Ok(order_by_option)
}

fn build_limit_options(limit: &str) -> Result<String, BuilderError> {
  let limit_option = utils::build_string_from_parts(vec![constants::LIMIT, limit]);

  Ok(limit_option)
}

fn build_offset_options(offset: &str) -> Result<String, BuilderError> {
  let offset_option = utils::build_string_from_parts(vec![constants::OFFSET, offset]);

  Ok(offset_option)
}

fn build_group_by(group_by: &Vec<String>) -> Result<String, BuilderError> {
  if group_by.is_empty() {
    return Ok(String::new());
  }

  let group_by_clause = group_by.join(", ");

  let result = format!("{} {}", constants::GROUP_BY, group_by_clause);
  Ok(result)
}

pub fn build_options(options: QueryOptions) -> Result<String, BuilderError> {
  let mut options_string = String::new();

  if let Some(ref order_by) = options.order_by {
    let order_by = match build_order_by(&order_by) {
      Ok(order_by) => order_by,
      Err(err) => return Err(err),
    };

    options_string.push_str(order_by.as_str());
  }

  if let Some(ref limit) = options.limit {
    let limit = match build_limit_options(limit) {
      Ok(limit) => limit,
      Err(err) => return Err(err),
    };

    options_string.push_str(limit.as_str());
  }

  if let Some(ref offset) = options.offset {
    let offset = match build_offset_options(offset) {
      Ok(offset) => offset,
      Err(err) => return Err(err),
    };

    options_string.push_str(offset.as_str());
  }

  if let Some(ref group_by) = options.group_by {
    let group_by = match build_group_by(group_by) {
      Ok(group_by) => group_by,
      Err(err) => return Err(err),
    };

    options_string.push_str(group_by.as_str());
  }

  Ok(options_string)
}
