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
mod counter;
pub use self::counter::*;

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
  Bool(bool),
  Str(String),
  Int(isize),
  Date(DateTime<Utc>),
  Map(HashMap<String, Value>),
  List(Vec<Value>),
}

impl Hash for Value {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      Value::Bool(b) => b.hash(state),
      Value::Str(s) => s.hash(state),
      Value::Int(i) => i.hash(state),
      Value::Date(d) => d.hash(state),
      Value::Map(h) => h.keys().collect::<Vec<&String>>().sort().hash(state),
      Value::List(v) => v.hash(state),
    };
  }
}

pub type Record = HashMap<String, Value>;
pub type Singleton<T> = std::rc::Rc<std::cell::RefCell<T>>;

#[macro_export]
macro_rules! singleton_new {
  ( $s:expr ) => {
    std::rc::Rc::new(std::cell::RefCell::new($s))
  };
}

#[macro_export]
macro_rules! singleton_share {
  ( $s:expr ) => {
    $s.clone()
  };
}

#[macro_export]
macro_rules! singleton_borrow {
  ( $s:expr ) => {
    (*$s).borrow_mut()
  };
}

#[cfg(test)]
mod test_util;
