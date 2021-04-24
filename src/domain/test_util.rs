use crate::domain::{
  Action, Chain, Config, CounterData, CounterRef, CountersPort, DnatMapping, DnatMappingsPort,
  Error, Filter, LogMessage, LogPort, Record, Singleton, Value,
};
use indexmap::IndexMap;
use std::{collections::HashMap, io::Write};

pub const ACT_NAME: &str = "fake_action";
pub const FLT_NAME: &str = "fake_filter";

impl Config {
  pub fn new(
    actions: Option<IndexMap<String, Chain>>,
    options: Option<HashMap<String, Value>>,
  ) -> Config {
    Config {
      actions: actions.unwrap_or(IndexMap::new()),
      options: options.unwrap_or(HashMap::new()),
    }
  }
}

pub struct FakeAction {}

impl Action for FakeAction {
  fn act(&mut self, record: &mut Record) -> Result<(), Error> {
    let v = record.get(ACT_NAME).unwrap_or(&Value::Int(0)).clone();
    match v {
      Value::Int(i) => record.insert(String::from(ACT_NAME), Value::Int(i + 1)),
      _ => panic!("The record did not contain the expected value."),
    };
    Ok(())
  }
}

pub struct FakeFilter {}

impl Filter for FakeFilter {
  fn filter(&mut self, record: &mut Record) -> bool {
    let v = record.get(FLT_NAME).unwrap_or(&Value::Int(0)).clone();
    match v {
      Value::Int(i) => record.insert(String::from(FLT_NAME), Value::Int(i + 1)),
      _ => panic!("The record did not contain the expected value."),
    };
    false
  }
}

pub struct FakeLog {
  pub wanted_next: Vec<Result<Record, Error>>,
  pub last_write: Option<(String, String)>,
}

impl FakeLog {
  pub fn new(wanted_next: Vec<Result<Record, Error>>) -> FakeLog {
    FakeLog {
      wanted_next,
      last_write: None,
    }
  }
}

impl LogPort for FakeLog {
  fn read_next(&mut self) -> Result<Record, Error> {
    if self.wanted_next.is_empty() {
      Err("ERROR!".into())
    } else {
      self.wanted_next.remove(0)
    }
  }

  fn write(&mut self, message: LogMessage) -> Result<(), Error> {
    self.last_write = match message {
      LogMessage::EMERG(m) => Some(("EMERG".into(), m.into())),
      LogMessage::ALERT(m) => Some(("ALERT".into(), m.into())),
      LogMessage::CRIT(m) => Some(("CRIT".into(), m.into())),
      LogMessage::ERR(m) => Some(("ERR".into(), m.into())),
      LogMessage::WARNING(m) => Some(("WARNING".into(), m.into())),
      LogMessage::NOTICE(m) => Some(("NOTICE".into(), m.into())),
      LogMessage::INFO(m) => Some(("INFO".into(), m.into())),
      LogMessage::DEBUG(m) => Some(("DEBUG".into(), m.into())),
    };
    Ok(())
  }
}

pub struct FakeCountersAdapter {
  pub counters: Singleton<HashMap<(String, Value), CounterData>>,
}
impl CountersPort for FakeCountersAdapter {
  fn modify(
    &mut self,
    entry: CounterRef,
    data: CounterData,
    mut f: impl FnMut(&mut CounterData, CounterData) -> usize,
  ) -> usize {
    let k = (entry.0.to_string(), entry.1.clone());
    if !singleton_borrow!(self.counters).contains_key(&k) {
      singleton_borrow!(self.counters).insert(k.clone(), (0, None));
    }
    f(singleton_borrow!(self.counters).get_mut(&k).unwrap(), data)
  }
  fn remove(&mut self, entry: CounterRef) -> Option<CounterData> {
    let k = (entry.0.to_string(), entry.1.clone());
    singleton_borrow!(self.counters).remove(&k)
  }
  fn remove_if(&mut self, predicate: impl Fn(&CounterData) -> bool) {
    singleton_borrow!(self.counters).retain(|_, v| !predicate(v));
  }
}

pub struct FakeDnatMappings {
  pub mappings: Vec<DnatMapping>,
}
impl DnatMappingsPort for FakeDnatMappings {
  fn put(&mut self, mapping: DnatMapping) {
    self.mappings.push(mapping);
  }
  fn get_all(&mut self) -> Vec<&DnatMapping> {
    self.mappings.iter().collect()
  }
}

pub struct WriteProxy<'t, W: Write> {
  inner: &'t mut W,
}
impl<'t, W: Write> WriteProxy<'t, W> {
  pub fn new<'x>(inner: &'x mut W) -> WriteProxy<'x, W> {
    WriteProxy { inner }
  }
}

impl<'t, W: Write> Write for WriteProxy<'t, W> {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    self.inner.write(buf)
  }
  fn flush(&mut self) -> std::io::Result<()> {
    self.inner.flush()
  }
}
