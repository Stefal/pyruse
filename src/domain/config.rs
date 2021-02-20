use crate::domain::{ModuleArgs, Value};
use indexmap::IndexMap;
use std::collections::HashMap;

pub trait ConfigPort {
  fn get(&self) -> &Config;
}

pub struct Config {
  pub actions: IndexMap<String, Chain>,
  pub options: HashMap<String, Value>,
}

pub type Chain = Vec<Step>;

pub struct Step {
  pub module: String,
  pub args: ModuleArgs,
  pub then_dest: Option<String>,
  pub else_dest: Option<String>,
}
