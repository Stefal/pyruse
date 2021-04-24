use crate::domain::{Error, Record};

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

pub trait LogPort {
  fn read_next(&mut self) -> Result<Record, Error>;
  fn write(&mut self, message: LogMessage) -> Result<(), Error>;
}
