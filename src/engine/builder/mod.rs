/*******************************************************************
* BUILDER - Entry point for building a query from a Parsed Statement
* This function serves as an entry point for building a query from a
* Parsed Statement. It takes a `ParsedStatement` object as input,
* extracts the operation to be performed, and delegates the building
* process to the appropriate module based on the operation. If the
* operation is not recognized, the function returns an `Ok` Result
* with a generic string.
* @param statement: ParsedStatement
********************************************************************/
use self::{
  error::BuilderError,
  read::find,
  write::{delete, insert, update},
};
use crate::parser::parsed::{ParsedOperation, ParsedStatement};

mod arguments;
mod builder;
mod error;
mod fields;
mod utils;

pub mod aggregations;
pub mod options;
pub mod read;
pub mod write;

pub fn build(statement: &ParsedStatement) -> Result<String, BuilderError> {
  match statement.operation {
    ParsedOperation::FindFirst => find::unique(&statement.query),
    ParsedOperation::FindMany => find::many(&statement.query),
    ParsedOperation::DeleteOne => delete::unique(&statement.query),
    ParsedOperation::DeleteMany => delete::many(&statement.query),
    ParsedOperation::InsertOne => insert::one(&statement.query),
    ParsedOperation::UpdateOne => update::one(&statement.query),
    ParsedOperation::UpdateMany => update::many(&statement.query),
    ParsedOperation::InsertMany => insert::many(&statement.query),
    ParsedOperation::Count => aggregations::aggregations::count(&statement.query),
    ParsedOperation::Average => aggregations::aggregations::average(&statement.query),
  }
}
