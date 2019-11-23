use crate::common::Record;
use crate::modules::{ModuleArgs,ModuleType};

pub struct Config<'a> {
  actions: Vec<Chain<'a>>,
  options: Record<'a>
}

pub struct Chain<'a> {
  name: String,
  steps: Vec<Step<'a>>
}

pub struct Step<'a> {
  module_name: String,
  module_type: ModuleType,
  args: ModuleArgs<'a>
}
