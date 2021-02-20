use crate::domain::Record;

#[derive(Debug)]
pub enum LogMessage<'t> {
  EMERG(&'t str),
  ALERT(&'t str),
  CRIT(&'t str),
  ERR(&'t str),
  WARNING(&'t str),
  NOTICE(&'t str),
  INFO(&'t str),
  DEBUG(&'t str),
}

pub trait LogPort: Sized {
  fn open() -> Result<Self, ()>;
  fn read_next(&mut self) -> Result<Record, ()>;
  fn write(&mut self, message: LogMessage) -> Result<(), ()>;
}
