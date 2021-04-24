mod data;
pub use self::data::*;
mod ipset;
pub use self::ipset::*;
mod nftables;
pub use self::nftables::*;

use crate::domain::{Config, Error, Value};
use std::ffi::OsString;
use std::str::FromStr;

fn read_command_from_options(
  conf: &Config,
  option_name: &str,
  command_entry: &str,
  default_command: &[&str],
) -> Result<(OsString, Vec<OsString>), Error> {
  let mut params = conf.options.get(option_name).and_then(|o| match o {
    Value::Map(m) => m.get(command_entry),
    _ => None,
  })
  .and_then(|n| match n {
    Value::Str(s) => Some(vec!(OsString::from_str(s).expect(&format!("“{}” could not be read", s)))),
    Value::List(l) => Some(l.iter().map(|v| match v {
      Value::Str(s) => OsString::from_str(s).expect(&format!("“{}” could not be read", s)),
      _ => panic!(format!("One item in the “{}” command-line components of {} options is not a shell command or a parameter", command_entry, option_name)),
    }).collect()),
    _ => None,
  })
  .filter(|v| !v.is_empty())
  .unwrap_or(default_command.iter().map(|s| OsString::from_str(s).unwrap()).collect());
  let cmd = params.remove(0);
  Ok((cmd, params))
}

#[cfg(test)]
mod tests {
  use super::read_command_from_options;
  use crate::domain::{Config, Value};
  use std::collections::HashMap;
  use std::ffi::OsString;

  #[test]
  fn default_command_is_used_if_unspecified() {
    let conf = Config::new(None, None);
    let (cmd, params) = read_command_from_options(&conf, "o", "c", &["cmd", "p1", "p2"]).unwrap();
    assert_eq!(OsString::from("cmd"), cmd);
    assert_eq!(vec![OsString::from("p1"), OsString::from("p2")], params);
  }

  #[test]
  fn command_cannot_be_an_empty_list_or_default_command_is_used() {
    let conf = test_config(&[]);
    let (cmd, params) = read_command_from_options(&conf, "o", "c", &["cmd"]).unwrap();
    assert_eq!(OsString::from("cmd"), cmd);
    assert_eq!(Vec::<OsString>::new(), params);
  }

  #[test]
  fn custom_commands_are_accepted() {
    let conf = test_config(&["a", "b"]);
    let (cmd, params) = read_command_from_options(&conf, "o", "c", &["cmd", "p"]).unwrap();
    assert_eq!(OsString::from("a"), cmd);
    assert_eq!(vec![OsString::from("b")], params);
  }

  fn test_config(command: &[&str]) -> Config {
    let mut options = HashMap::new();
    let mut cmd_opt = HashMap::new();
    cmd_opt.insert(
      "c".into(),
      Value::List(command.iter().map(|s| Value::Str(s.to_string())).collect()),
    );
    options.insert("o".into(), Value::Map(cmd_opt));
    Config::new(None, Some(options))
  }
}
