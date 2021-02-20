use crate::domain::{Action, AvailableAction, AvailableFilter, Filter, ModulesPort, Record, Value};
use std::collections::HashMap;

pub const ACT_NAME: &str = "fake_action";
pub const FLT_NAME: &str = "fake_filter";

#[derive(Debug)]
pub struct FakeAction {}

impl Action for FakeAction {
  fn act(&self, record: &mut Record) -> Result<(), ()> {
    let v = record.get(ACT_NAME).unwrap_or(&Value::Int(0));
    match v {
      Value::Int(i) => record.insert(String::from(ACT_NAME), Value::Int(i + 1)),
      _ => panic!("The record did not contain the expected value."),
    };
    Ok(())
  }
}

#[derive(Debug)]
pub struct FakeFilter {}

impl Filter for FakeFilter {
  fn filter(&self, record: &mut Record) -> bool {
    let v = record.get(FLT_NAME).unwrap_or(&Value::Int(0));
    match v {
      Value::Int(i) => record.insert(String::from(FLT_NAME), Value::Int(i + 1)),
      _ => panic!("The record did not contain the expected value."),
    };
    false
  }
}

pub struct FakeModulesAdapter<'a> {
  a: HashMap<&'a String, &'a AvailableAction>,
  f: HashMap<&'a String, &'a AvailableFilter>,
}

impl FakeModulesAdapter<'_> {
  pub fn new<'a>(act: &'a [AvailableAction], flt: &'a [AvailableFilter]) -> FakeModulesAdapter<'a> {
    let a = act
      .iter()
      .map(|m| (&m.name, m))
      .collect::<HashMap<&'a String, &'a AvailableAction>>();
    let f = flt
      .iter()
      .map(|m| (&m.name, m))
      .collect::<HashMap<&'a String, &'a AvailableFilter>>();
    FakeModulesAdapter { a, f }
  }
}

impl ModulesPort for FakeModulesAdapter<'_> {
  fn available_actions(&self) -> HashMap<&String, &AvailableAction> {
    self.a.clone()
  }
  fn available_filters(&self) -> HashMap<&String, &AvailableFilter> {
    self.f.clone()
  }
}
