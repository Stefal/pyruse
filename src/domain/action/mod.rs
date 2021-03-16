mod counter_raise;
pub use self::counter_raise::*;
mod counter_reset;
pub use self::counter_reset::*;
mod dnat_capture;
pub use self::dnat_capture::*;
mod dnat_replace;
pub use self::dnat_replace::*;
mod log;
pub use self::log::*;
mod noop;
pub use self::noop::*;

use crate::domain::{Counters, CountersPort, ModuleArgs, Record, Singleton, Value};
use chrono::Duration;

pub struct CounterAction<C: CountersPort> {
  counters: Singleton<Counters<C>>,
  counter_name: String,
  counter_key: String,
  save_into: Option<String>,
  duration: Option<Duration>,
}

impl<C: CountersPort> CounterAction<C> {
  pub fn from_args<X: CountersPort>(
    mut args: ModuleArgs,
    counters: Singleton<Counters<X>>,
    action_name: &str,
    duration_name: &str,
  ) -> CounterAction<X> {
    let counter_name = remove_acceptable_key(&mut args, "counter").expect(&format!(
      "The {} action needs a counter name in “counter”",
      action_name
    ));
    let counter_key = remove_acceptable_key(&mut args, "for").expect(&format!(
      "The {} action needs a counter key in “for”",
      action_name
    ));
    let save_into = remove_acceptable_key(&mut args, "save");
    let duration = match args.remove(duration_name) {
      None => None,
      Some(Value::Int(i)) => Some(Duration::seconds(i as i64)),
      _ => panic!(format!(
        "The {} only accepts a number of seconds in “{}”",
        action_name, duration_name
      )),
    };
    CounterAction {
      counters,
      counter_name,
      counter_key,
      save_into,
      duration,
    }
  }
}

pub fn get_acceptable_key<'r, 'k>(record: &'r Record, key: &'k str) -> Option<String> {
  match record.get(key) {
    Some(&Value::Str(ref s)) => Some(s.clone()),
    Some(&Value::Int(ref i)) => Some(format!("{}", i)),
    Some(&Value::Date(ref d)) => Some(format!("{}", d.timestamp())),
    _ => None,
  }
}

pub fn remove_acceptable_key(args: &mut ModuleArgs, key: &str) -> Option<String> {
  match args.remove(key) {
    Some(Value::Str(s)) => Some(s),
    Some(Value::Int(i)) => Some(format!("{}", i)),
    Some(Value::Date(d)) => Some(format!("{}", d.timestamp())),
    _ => None,
  }
}
