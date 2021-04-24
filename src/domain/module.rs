use crate::domain::{Error, Record, Value};
use std::collections::HashMap;
pub type ActionConstructor = Box<dyn Fn(ModuleArgs) -> Box<dyn Action>>;
pub type FilterConstructor = Box<dyn Fn(ModuleArgs) -> Box<dyn Filter>>;

pub struct Modules {
  available_actions: HashMap<String, ActionConstructor>,
  available_filters: HashMap<String, FilterConstructor>,
}

impl Modules {
  pub fn new() -> Modules {
    Modules {
      available_actions: HashMap::new(),
      available_filters: HashMap::new(),
    }
  }

  pub fn register_action(&mut self, name: String, cons: ActionConstructor) {
    self.available_actions.insert(name, cons);
  }

  pub fn register_filter(&mut self, name: String, cons: FilterConstructor) {
    self.available_filters.insert(name, cons);
  }
}

pub enum Module {
  Action(Box<dyn Action>),
  Filter(Box<dyn Filter>),
}

impl Module {
  pub fn new(name: &str, args: ModuleArgs, available: &Modules) -> Result<Module, Error> {
    if let Some(a) = available.available_actions.get(name) {
      Ok(Module::Action(a(args)))
    } else if let Some(f) = available.available_filters.get(name) {
      Ok(Module::Filter(f(args)))
    } else {
      Err(format!("Module {} does not exist", name).into())
    }
  }

  pub fn run(&mut self, record: &mut Record) -> Result<bool, Error> {
    match self {
      Module::Action(a) => a.act(record).map(|_| true),
      Module::Filter(f) => Ok(f.filter(record)),
    }
  }
}

pub trait Action {
  fn act(&mut self, record: &mut Record) -> Result<(), Error>;
}

pub trait Filter {
  fn filter(&mut self, record: &mut Record) -> bool;
}

pub type ModuleArgs = HashMap<String, Value>;

#[cfg(test)]
mod tests {
  use super::{Module, Modules, Record, Value};
  use crate::domain::test_util::{FakeAction, FakeFilter, ACT_NAME, FLT_NAME};
  use std::collections::HashMap;

  #[test]
  fn available_action_can_be_generated_and_run() {
    // Given
    let mut mods = Modules::new();
    mods.register_action(ACT_NAME.to_string(), Box::new(|_| Box::new(FakeAction {})));
    let mut record: Record = HashMap::new();

    // When
    let mut module = Module::new(ACT_NAME, HashMap::new(), &mods).unwrap();

    // Then
    assert_eq!(Ok(true), module.run(&mut record));
    assert!(record.contains_key(ACT_NAME));
    assert_eq!(record[ACT_NAME], Value::Int(1));
  }

  #[test]
  fn available_filter_can_be_generated_and_run() {
    // Given
    let mut mods = Modules::new();
    mods.register_filter(FLT_NAME.to_string(), Box::new(|_| Box::new(FakeFilter {})));
    let mut record: Record = HashMap::new();

    // When
    let mut module = Module::new(FLT_NAME, HashMap::new(), &mods).unwrap();

    // Then
    assert_eq!(Ok(false), module.run(&mut record));
    assert!(record.contains_key(FLT_NAME));
    assert_eq!(record[FLT_NAME], Value::Int(1));
  }
}
