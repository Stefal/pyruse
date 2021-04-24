use crate::domain::{
  Action, Error, LogMessage, LogPort, ModuleArgs, Record, Singleton, Template, Value,
};
use crate::singleton_borrow;

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
  logger: Singleton<dyn LogPort>,
  log_format: LogFormat,
  template: Template,
}

impl Log {
  pub fn from_args(mut args: ModuleArgs, logger: Singleton<dyn LogPort>) -> Log {
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
    let template = Template::new(match args.remove("message") {
      Some(Value::Str(s)) => s,
      _ => panic!("The Log action needs a message template in “message”"),
    });
    Log {
      logger,
      log_format,
      template,
    }
  }
}

impl Action for Log {
  fn act(&mut self, record: &mut Record) -> Result<(), Error> {
    let message = self.template.format(record);
    singleton_borrow!(self.logger).write((self.log_format)(&message))
  }
}

#[cfg(test)]
mod tests {
  use super::Log;
  use crate::domain::test_util::FakeLog;
  use crate::domain::{Action, Error, ModuleArgs, Record, Singleton, Value};
  use crate::{assert_log_match, singleton_new, singleton_share};
  use core::panic;
  use std::collections::HashMap;

  #[test]
  #[should_panic(expected = "The Log action needs a message template in “message”")]
  fn arg_message_is_mandatory() {
    let args: ModuleArgs = HashMap::new();
    let logger = singleton_new!(FakeLog::new(Vec::new()));
    Log::from_args(args, singleton_share!(logger));
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
    logs: Vec<Result<Record, Error>>,
    vars: Vec<(&str, &str)>,
  ) -> (Log, Singleton<FakeLog>, Record) {
    let mut args: ModuleArgs = HashMap::new();
    args.insert("message".into(), Value::Str(template.to_string()));
    if let Some(l) = level {
      args.insert("level".into(), Value::Str(l.to_string()));
    }
    let logger = singleton_new!(FakeLog::new(logs));
    let log = Log::from_args(args, singleton_share!(logger));
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
