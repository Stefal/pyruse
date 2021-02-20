use crate::domain::{Record, Value};
use std::collections::HashMap;
use std::fmt::Debug;

pub struct AvailableAction {
  pub name: String,
  pub cons: fn(ModuleArgs) -> Box<dyn Action>,
}

impl AvailableAction {
  pub fn new(name: String, cons: fn(ModuleArgs) -> Box<dyn Action>) -> Self {
    AvailableAction { name, cons }
  }
}

pub struct AvailableFilter {
  pub name: String,
  pub cons: fn(ModuleArgs) -> Box<dyn Filter>,
}

impl AvailableFilter {
  pub fn new(name: String, cons: fn(ModuleArgs) -> Box<dyn Filter>) -> Self {
    AvailableFilter { name, cons }
  }
}

pub trait ModulesPort {
  fn available_actions(&self) -> HashMap<&String, &AvailableAction>;
  fn available_filters(&self) -> HashMap<&String, &AvailableFilter>;
}

#[derive(Debug)]
pub enum Module {
  Action(Box<dyn Action>),
  Filter(Box<dyn Filter>),
}

impl Module {
  pub fn new(name: String, args: ModuleArgs, available: &dyn ModulesPort) -> Result<Module, ()> {
    if let Some(a) = available.available_actions().get(&name).map(|m| m.cons) {
      Ok(Module::Action(a(args)))
    } else if let Some(f) = available.available_filters().get(&name).map(|m| m.cons) {
      Ok(Module::Filter(f(args)))
    } else {
      Err(())
    }
  }

  pub fn run(&self, record: &mut Record) -> Result<bool, ()> {
    match self {
      Module::Action(a) => match a.act(record) {
        Ok(()) => Ok(true),
        Err(()) => Err(()),
      },
      Module::Filter(f) => Ok(f.filter(record)),
    }
  }
}

pub trait Action: Debug {
  fn act(&self, record: &mut Record) -> Result<(), ()>;
}

pub trait Filter: Debug {
  fn filter(&self, record: &mut Record) -> bool;
}

pub type ModuleArgs = HashMap<String, Value>;

#[cfg(test)]
mod tests {
  use super::{AvailableAction, AvailableFilter, Module, Record, Value};
  use crate::domain::test_util::*;
  use std::collections::HashMap;

  #[test]
  fn available_action_can_be_generated_and_run() {
    // Given
    let aa = [AvailableAction {
      name: ACT_NAME.to_string(),
      cons: |_| Box::new(FakeAction {}),
    }];
    let mut record: Record = HashMap::new();
    let mods = FakeModulesAdapter::new(&aa, &[]);

    // When
    let module = Module::new(ACT_NAME.to_string(), HashMap::new(), &mods).unwrap();

    // Then
    assert!(module.run(&mut record) == Ok(true));
    assert!(record.contains_key(ACT_NAME));
    assert!(record[ACT_NAME] == Value::Int(1));
  }

  #[test]
  fn available_filter_can_be_generated_and_run() {
    // Given
    let af = [AvailableFilter {
      name: FLT_NAME.to_string(),
      cons: |_| Box::new(FakeFilter {}),
    }];
    let mut record: Record = HashMap::new();
    let mods = FakeModulesAdapter::new(&[], &af);

    // When
    let module = Module::new(FLT_NAME.to_string(), HashMap::new(), &mods).unwrap();

    // Then
    assert!(module.run(&mut record) == Ok(false));
    assert!(record.contains_key(FLT_NAME));
    assert!(record[FLT_NAME] == Value::Int(1));
  }
}
