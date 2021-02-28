use crate::domain::{Action, Filter, LogMessage, LogPort, Record, Value};

pub const ACT_NAME: &str = "fake_action";
pub const FLT_NAME: &str = "fake_filter";

pub struct FakeAction {}

impl Action for FakeAction {
  fn act(&mut self, record: &mut Record) -> Result<(), ()> {
    let v = record.get(ACT_NAME).unwrap_or(&Value::Int(0));
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
    let v = record.get(FLT_NAME).unwrap_or(&Value::Int(0));
    match v {
      Value::Int(i) => record.insert(String::from(FLT_NAME), Value::Int(i + 1)),
      _ => panic!("The record did not contain the expected value."),
    };
    false
  }
}

pub struct FakeLog {
  pub wanted_next: Vec<Result<Record, ()>>,
  pub last_write: Option<(String, String)>,
}

impl FakeLog {
  pub fn new(wanted_next: Vec<Result<Record, ()>>) -> FakeLog {
    FakeLog {
      wanted_next,
      last_write: None,
    }
  }
}

impl LogPort for FakeLog {
  fn read_next(&mut self) -> Result<Record, ()> {
    if self.wanted_next.is_empty() {
      Err(())
    } else {
      self.wanted_next.remove(0)
    }
  }

  fn write(&mut self, message: LogMessage) -> Result<(), ()> {
    self.last_write = match message {
      LogMessage::EMERG(m) => Some(("EMERG".to_string(), m.to_string())),
      LogMessage::ALERT(m) => Some(("ALERT".to_string(), m.to_string())),
      LogMessage::CRIT(m) => Some(("CRIT".to_string(), m.to_string())),
      LogMessage::ERR(m) => Some(("ERR".to_string(), m.to_string())),
      LogMessage::WARNING(m) => Some(("WARNING".to_string(), m.to_string())),
      LogMessage::NOTICE(m) => Some(("NOTICE".to_string(), m.to_string())),
      LogMessage::INFO(m) => Some(("INFO".to_string(), m.to_string())),
      LogMessage::DEBUG(m) => Some(("DEBUG".to_string(), m.to_string())),
    };
    Ok(())
  }
}
