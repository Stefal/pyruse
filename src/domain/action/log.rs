use crate::domain::{Action, LogMessage, LogPort, ModuleArgs, Record, Value};
use regex::Regex;
use std::cell::RefCell;
use std::ops::{Index, Range};
use std::rc::Rc;

type LogFormat = fn(&str) -> LogMessage;

const EMERG_LOG_FORMAT: LogFormat = |s| LogMessage::EMERG(&s);
const ALERT_LOG_FORMAT: LogFormat = |s| LogMessage::ALERT(&s);
const CRIT_LOG_FORMAT: LogFormat = |s| LogMessage::CRIT(&s);
const ERR_LOG_FORMAT: LogFormat = |s| LogMessage::ERR(&s);
const WARNING_LOG_FORMAT: LogFormat = |s| LogMessage::WARNING(&s);
const NOTICE_LOG_FORMAT: LogFormat = |s| LogMessage::NOTICE(&s);
const INFO_LOG_FORMAT: LogFormat = |s| LogMessage::INFO(&s);
const DEBUG_LOG_FORMAT: LogFormat = |s| LogMessage::DEBUG(&s);

pub struct Log {
  logger: Rc<RefCell<dyn LogPort>>,
  log_format: LogFormat,
  template: String,
  var_locations: Vec<Range<usize>>,
}

impl Log {
  pub fn from_args(mut args: ModuleArgs, logger: Rc<RefCell<dyn LogPort>>) -> Log {
    let log_format = match args.remove("level") {
      Some(Value::Str(l)) => match l.as_ref() {
        "EMERG" => EMERG_LOG_FORMAT,
        "ALERT" => ALERT_LOG_FORMAT,
        "CRIT" => CRIT_LOG_FORMAT,
        "ERR" => ERR_LOG_FORMAT,
        "WARNING" => WARNING_LOG_FORMAT,
        "NOTICE" => NOTICE_LOG_FORMAT,
        "INFO" => INFO_LOG_FORMAT,
        "DEBUG" => DEBUG_LOG_FORMAT,
        _ => {
          eprintln!("Unknown error level: {}; Using INFO", l);
          INFO_LOG_FORMAT
        }
      },
      _ => INFO_LOG_FORMAT,
    };
    let template = match args.remove("message") {
      Some(Value::Str(s)) => s,
      _ => panic!("The Log action needs a message template in “message”"),
    };
    let var_locations = Regex::new(r"\{\w+\}")
      .unwrap()
      .find_iter(&template)
      .map(|m| m.range())
      .collect();
    Log {
      logger,
      log_format,
      template,
      var_locations,
    }
  }

  fn message_with_variables_from_record(&self, record: &mut Record) -> String {
    let tpl = &self.template;
    if self.var_locations.len() == 0 {
      return tpl.clone();
    }
    let mut message = String::with_capacity(tpl.len() * 2);
    let mut last_index = 0;
    for Range { start, end } in self.var_locations.iter() {
      message.push_str(tpl.index(last_index..*start));
      if let Some(Value::Str(s)) = record.get(tpl.index((start + 1)..(end - 1))) {
        message.push_str(&s);
      } else {
        message.push_str(tpl.index(*start..*end));
      }
      last_index = *end;
    }
    message.push_str(tpl.index(last_index..tpl.len()));
    message
  }
}

impl Action for Log {
  fn act(&mut self, record: &mut Record) -> Result<(), ()> {
    let message = self.message_with_variables_from_record(record);
    (self.logger.borrow_mut()).write((self.log_format)(&message))
  }
}

#[cfg(test)]
#[macro_use]
mod tests {
  use super::Log;
  use crate::assert_log_match;
  use crate::domain::test_util::*;
  use crate::domain::{Action, ModuleArgs, Record, Value};
  use core::panic;
  use std::cell::RefCell;
  use std::collections::HashMap;
  use std::rc::Rc;

  #[test]
  #[should_panic(expected = "The Log action needs a message template in “message”")]
  fn arg_message_is_mandatory() {
    let args: ModuleArgs = HashMap::new();
    let logger = Rc::new(RefCell::new(FakeLog::new(Vec::new())));
    Log::from_args(args, logger.clone());
  }

  #[test]
  fn default_log_level_is_info() {
    let (mut log, logger, mut record) = create_log_logger_record("x", None, Vec::new(), Vec::new());
    log.act(&mut record).unwrap();
    assert_log_match!(logger, Some((ref l, _)), l == "INFO");
  }

  #[test]
  fn test_levels_are_recognized() {
    let mut levels = vec![
      "EMERG", "ALERT", "CRIT", "ERR", "WARNING", "NOTICE", "INFO", "DEBUG",
    ];
    levels.iter_mut().for_each(|level| {
      let (mut log, logger, mut record) =
        create_log_logger_record("x", Some(level), Vec::new(), Vec::new());
      log.act(&mut record).unwrap();
      assert_log_match!(logger, Some((ref l, _)), l == level);
    });
  }

  #[test]
  fn template_without_placeholder_is_rendered_as_is() {
    let (mut log, logger, mut record) = create_log_logger_record("x", None, Vec::new(), Vec::new());
    log.act(&mut record).unwrap();
    assert_log_match!(logger, Some((_, ref m)), m == "x");
  }

  #[test]
  fn placeholder_without_variable_is_rendered_as_is() {
    let (mut log, logger, mut record) =
      create_log_logger_record("x{y}z", None, Vec::new(), Vec::new());
    log.act(&mut record).unwrap();
    assert_log_match!(logger, Some((_, ref m)), m == "x{y}z");
  }

  #[test]
  fn placeholder_with_variable_is_replaced() {
    let (mut log, logger, mut record) =
      create_log_logger_record("x{y}z", None, Vec::new(), vec![("y", "y")]);
    log.act(&mut record).unwrap();
    assert_log_match!(logger, Some((_, ref m)), m == "xyz");
  }

  #[test]
  fn all_variables_are_replaced() {
    let (mut log, logger, mut record) = create_log_logger_record(
      "{x}{a}{yy}-{zzz}",
      None,
      Vec::new(),
      vec![("x", "x"), ("yy", "y"), ("zzz", "z")],
    );
    log.act(&mut record).unwrap();
    assert_log_match!(logger, Some((_, ref m)), m == "x{a}y-z");
  }

  fn create_log_logger_record(
    template: &str,
    level: Option<&str>,
    logs: Vec<Result<Record, ()>>,
    vars: Vec<(&str, &str)>,
  ) -> (Log, Rc<RefCell<FakeLog>>, Record) {
    let mut args: ModuleArgs = HashMap::new();
    args.insert("message".to_string(), Value::Str(template.to_string()));
    if let Some(l) = level {
      args.insert("level".to_string(), Value::Str(l.to_string()));
    }
    let logger = Rc::new(RefCell::new(FakeLog::new(logs)));
    let log = Log::from_args(args, logger.clone());
    let mut record: Record = HashMap::new();
    for (name, value) in vars {
      record.insert(name.to_string(), Value::Str(value.to_string()));
    }
    (log, logger, record)
  }

  #[macro_export]
  macro_rules! assert_log_match {
    ( $l:ident , $x:pat , $y:expr ) => {
      let logger_contents = (*$l).borrow_mut();
      if let $x = logger_contents.last_write {
        assert!($y);
      } else {
        panic!("There should be a log entry in this test.");
      }
    };
  }
}
