use crate::domain::{Error, LogMessage, LogPort, Record, Value};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::iter::FromIterator;
use systemd::journal::{print, Journal, OpenOptions};

type JournalFieldMapper = fn(String) -> Value;

const STR_MAPPER: JournalFieldMapper = |s| Value::Str(s);
const INT_MAPPER: JournalFieldMapper = |s| {
  s.parse::<isize>()
    .map(|i| Value::Int(i))
    .unwrap_or(Value::Str(s))
};
const DATE_MAPPER: JournalFieldMapper = |s| {
  s.parse::<DateTime<Utc>>()
    .map(|d| Value::Date(d))
    .unwrap_or(Value::Str(s))
};

pub struct SystemdLogAdapter {
  journal: Journal,
  mappers: HashMap<String, JournalFieldMapper>,
}

impl SystemdLogAdapter {
  pub fn open() -> Result<Self, Error> {
    let mut journal = OpenOptions::default()
      .system(true)
      .current_user(false)
      .local_only(false)
      .open()?;
    journal.seek_tail()?;
    let mappers = create_mappers();
    Ok(SystemdLogAdapter { journal, mappers })
  }
}

impl LogPort for SystemdLogAdapter {
  fn read_next(&mut self) -> Result<Record, Error> {
    loop {
      match self.journal.await_next_entry(None)? {
        Some(mut entry) => {
          let mut record: Record = HashMap::with_capacity(entry.len());
          let all_keys = entry.keys().map(|s| s.clone()).collect::<Vec<String>>();
          for k in all_keys {
            let (k, v) = entry.remove_entry(&k).unwrap();
            let mapper = self.mappers.get(&k);
            if mapper == None {
              continue;
            }
            record.insert(k, (mapper.unwrap())(v));
          }
          return Ok(record);
        }
        None => continue,
      };
    }
  }

  fn write(&mut self, message: LogMessage) -> Result<(), Error> {
    let unix_status = match message {
      LogMessage::EMERG(m) => print(0, m),
      LogMessage::ALERT(m) => print(1, m),
      LogMessage::CRIT(m) => print(2, m),
      LogMessage::ERR(m) => print(3, m),
      LogMessage::WARNING(m) => print(4, m),
      LogMessage::NOTICE(m) => print(5, m),
      LogMessage::INFO(m) => print(6, m),
      LogMessage::DEBUG(m) => print(7, m),
    };
    match unix_status {
      0 => Ok(()),
      _ => Err("Writing the systemd log resulted in a non-zero status".into()),
    }
  }
}

fn create_mappers<'t>() -> HashMap<String, JournalFieldMapper> {
  let map: HashMap<String, JournalFieldMapper> = HashMap::from_iter(
    [
      ("MESSAGE", STR_MAPPER),
      ("MESSAGE_ID", STR_MAPPER),
      ("PRIORITY", INT_MAPPER),
      ("CODE_FILE", STR_MAPPER),
      ("CODE_LINE", INT_MAPPER),
      ("CODE_FUNC", STR_MAPPER),
      ("ERRNO", INT_MAPPER),
      ("INVOCATION_ID", STR_MAPPER),
      ("USER_INVOCATION_ID", STR_MAPPER),
      ("SYSLOG_FACILITY", INT_MAPPER),
      ("SYSLOG_IDENTIFIER", STR_MAPPER),
      ("SYSLOG_PID", INT_MAPPER),
      ("SYSLOG_TIMESTAMP", DATE_MAPPER),
      ("SYSLOG_RAW", STR_MAPPER),
      ("DOCUMENTATION", STR_MAPPER),
      ("TID", INT_MAPPER),
      ("_PID", INT_MAPPER),
      ("_UID", INT_MAPPER),
      ("_GID", INT_MAPPER),
      ("_COMM", STR_MAPPER),
      ("_EXE", STR_MAPPER),
      ("_CMDLINE", STR_MAPPER),
      ("_CAP_EFFECTIVE", STR_MAPPER),
      ("_AUDIT_SESSION", STR_MAPPER),
      ("_AUDIT_LOGINUID", INT_MAPPER),
      ("_SYSTEMD_CGROUP", STR_MAPPER),
      ("_SYSTEMD_SLICE", STR_MAPPER),
      ("_SYSTEMD_UNIT", STR_MAPPER),
      ("_SYSTEMD_USER_UNIT", STR_MAPPER),
      ("_SYSTEMD_USER_SLICE", STR_MAPPER),
      ("_SYSTEMD_SESSION", STR_MAPPER),
      ("_SYSTEMD_OWNER_UID", INT_MAPPER),
      ("_SELINUX_CONTEXT", STR_MAPPER),
      ("_SOURCE_REALTIME_TIMESTAMP", DATE_MAPPER),
      ("_BOOT_ID", STR_MAPPER),
      ("_MACHINE_ID", STR_MAPPER),
      ("_SYSTEMD_INVOCATION_ID", STR_MAPPER),
      ("_HOSTNAME", STR_MAPPER),
      ("_TRANSPORT", STR_MAPPER),
      ("_STREAM_ID", STR_MAPPER),
      ("_LINE_BREAK", STR_MAPPER),
      ("_NAMESPACE", STR_MAPPER),
      ("_KERNEL_DEVICE", STR_MAPPER),
      ("_KERNEL_SUBSYSTEM", STR_MAPPER),
      ("_UDEV_SYSNAME", STR_MAPPER),
      ("_UDEV_DEVNODE", STR_MAPPER),
      ("_UDEV_DEVLINK", STR_MAPPER),
      ("COREDUMP_UNIT", STR_MAPPER),
      ("COREDUMP_USER_UNIT", STR_MAPPER),
      ("OBJECT_PID", INT_MAPPER),
      ("OBJECT_UID", INT_MAPPER),
      ("OBJECT_GID", INT_MAPPER),
      ("OBJECT_COMM", STR_MAPPER),
      ("OBJECT_EXE", STR_MAPPER),
      ("OBJECT_CMDLINE", STR_MAPPER),
      ("OBJECT_AUDIT_SESSION", STR_MAPPER),
      ("OBJECT_AUDIT_LOGINUID", INT_MAPPER),
      ("OBJECT_SYSTEMD_CGROUP", STR_MAPPER),
      ("OBJECT_SYSTEMD_SESSION", STR_MAPPER),
      ("OBJECT_SYSTEMD_OWNER_UID", INT_MAPPER),
      ("OBJECT_SYSTEMD_UNIT", STR_MAPPER),
      ("OBJECT_SYSTEMD_USER_UNIT", STR_MAPPER),
      ("__CURSOR", STR_MAPPER),
      ("__REALTIME_TIMESTAMP", DATE_MAPPER),
      ("__MONOTONIC_TIMESTAMP", DATE_MAPPER),
    ]
    .iter()
    .map(|(s, m)| ((*s).into(), m.to_owned())),
  );
  map
}
