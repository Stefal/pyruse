use chrono::DateTime;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
  Bool(bool),
  Str(String),
  Int(isize),
  Date(DateTime<chrono::Utc>)
}

pub type Record<'a> = HashMap<&'a str, Value>;
