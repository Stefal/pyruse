use super::CounterAction;
use crate::domain::{Action, Counters, CountersPort, ModuleArgs, Record, Singleton, Value};
use crate::singleton_borrow;
use chrono::Utc;

pub struct CounterRaise<C: CountersPort> {
  act: CounterAction<C>,
}

impl<C: CountersPort> CounterRaise<C> {
  pub fn from_args<X: CountersPort>(
    args: ModuleArgs,
    counters: Singleton<Counters<X>>,
  ) -> CounterRaise<X> {
    CounterRaise {
      act: CounterAction::<X>::from_args(args, counters, "CounterRaise", "keepSeconds"),
    }
  }
}

impl<C: CountersPort> Action for CounterRaise<C> {
  fn act(&mut self, record: &mut Record) -> Result<(), ()> {
    match record.get(&self.act.counter_key) {
      None => Err(()),
      Some(v) => {
        let count = singleton_borrow!(self.act.counters).augment(
          (self.act.counter_name.as_ref(), v),
          (1, self.act.duration.map(|d| Utc::now() + d)),
        );
        if let Some(s) = &self.act.save_into {
          record.insert(s.clone(), Value::Int(count as isize));
        };
        Ok(())
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::action::CounterRaise;
  use crate::domain::test_util::FakeCountersAdapter;
  use crate::domain::{Action, CounterData, Counters, Singleton, Value};
  use crate::{singleton_borrow, singleton_new, singleton_share};
  use chrono::{Duration, Utc};
  use std::collections::HashMap;
  use std::{thread, time};

  #[test]
  fn when_non_existing_then_raise_to_1() {
    let (_, mut action) = get_counters_action();
    let mut record = HashMap::with_capacity(1);
    record.insert("k".to_string(), Value::Str("raise#1".to_string()));

    action.act(&mut record).unwrap();
    assert_eq!(Some(&Value::Int(1)), record.get("raise"));
  }

  #[test]
  fn when_different_key_then_different_counter() {
    let (_, mut action) = get_counters_action();
    let mut record1 = HashMap::with_capacity(1);
    record1.insert("k".to_string(), Value::Str("raise#3".to_string()));
    let mut record2 = HashMap::with_capacity(1);
    record2.insert("k".to_string(), Value::Str("raise#4".to_string()));

    action.act(&mut record1).unwrap();
    assert_eq!(Some(&Value::Int(1)), record1.get("raise"));
    action.act(&mut record2).unwrap();
    assert_eq!(Some(&Value::Int(1)), record2.get("raise"));
    action.act(&mut record2).unwrap();
    assert_eq!(Some(&Value::Int(2)), record2.get("raise"));
    action.act(&mut record2).unwrap();
    assert_eq!(Some(&Value::Int(3)), record2.get("raise"));
    action.act(&mut record1).unwrap();
    assert_eq!(Some(&Value::Int(2)), record1.get("raise"));
  }

  #[test]
  fn when_grace_time_then_count_is_0() {
    let (counters, mut action) = get_counters_action();
    let mut record = HashMap::with_capacity(1);
    record.insert("k".to_string(), Value::Str("raise#5".to_string()));
    singleton_borrow!(counters).insert(
      ("test".to_string(), Value::Str("raise#5".to_string())),
      (0, Some(Utc::now() + Duration::seconds(1))),
    );

    action.act(&mut record).unwrap();
    assert_eq!(Some(&Value::Int(0)), record.get("raise"));
    thread::sleep(time::Duration::from_secs(1));
    action.act(&mut record).unwrap();
    assert_eq!(Some(&Value::Int(1)), record.get("raise"));
  }

  fn get_counters_action() -> (
    Singleton<HashMap<(String, Value), CounterData>>,
    CounterRaise<FakeCountersAdapter>,
  ) {
    let counters = singleton_new!(HashMap::new());
    let counters_backend =
      singleton_new!(Counters::<FakeCountersAdapter>::new(FakeCountersAdapter {
        counters: singleton_share!(counters)
      }));
    let mut args = HashMap::with_capacity(3);
    args.insert("counter".to_string(), Value::Str("test".to_string()));
    args.insert("for".to_string(), Value::Str("k".to_string()));
    args.insert("save".to_string(), Value::Str("raise".to_string()));
    let action = CounterRaise::<FakeCountersAdapter>::from_args(args, counters_backend);
    (counters, action)
  }
}
