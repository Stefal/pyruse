use std::collections::HashMap;
use crate::common::{Record,Value};

pub struct AvailableAction {
  name: &'static str,
  cons: fn(ModuleArgs) -> Box<dyn Action>
}

impl AvailableAction {
  pub fn new(name: &'static str, cons: fn(ModuleArgs) -> Box<dyn Action>) -> Self {
    AvailableAction { name, cons }
  }
}

inventory::collect!(AvailableAction);

pub struct AvailableFilter {
  name: &'static str,
  cons: fn(ModuleArgs) -> Box<dyn Filter>
}

impl AvailableFilter {
  pub fn new(name: &'static str, cons: fn(ModuleArgs) -> Box<dyn Filter>) -> Self {
    AvailableFilter { name, cons }
  }
}

inventory::collect!(AvailableFilter);

pub enum Module {
  Action(Box<dyn Action>),
  Filter(Box<dyn Filter>)
}

impl Module {
  pub fn get_module(name: &str, args: ModuleArgs) -> Result<Module, ()> {
    for action in inventory::iter::<AvailableAction> {
      if action.name == name {
        return Ok(Module::Action((action.cons)(args)))
      }
    }
    for filter in inventory::iter::<AvailableFilter> {
      if filter.name == name {
        return Ok(Module::Filter((filter.cons)(args)))
      }
    }
    Err(())
  }

  pub fn run(&self, record: &mut Record) -> Result<bool, ()> {
    match self {
      Module::Action(a) => match a.act(record) {
        Ok(()) => Ok(true),
        Err(()) => Err(())
      },
      Module::Filter(f) => Ok(f.filter(record))
    }
  }
}

pub trait Action {
  fn act(&self, record: &mut Record) -> Result<(), ()>;
}

pub trait Filter {
  fn filter(&self, record: &mut Record) -> bool;
}

pub type ModuleArgs = HashMap<String, Value>;
