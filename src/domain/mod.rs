pub mod action;
pub mod filter;

mod config;
pub use self::config::*;
mod log;
pub use self::log::*;
mod module;
pub use self::module::*;
mod workflow;
pub use self::workflow::*;

use chrono::DateTime;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
pub enum Value {
  Bool(bool),
  Str(String),
  Int(isize),
  Date(DateTime<chrono::Utc>),
  Map(HashMap<String, Value>),
  List(Vec<Value>),
}

pub type Record = HashMap<String, Value>;

#[cfg(test)]
mod test_util;
