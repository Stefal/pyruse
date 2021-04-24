use super::CounterAction;
use crate::domain::{Action, Counters, CountersPort, Error, ModuleArgs, Record, Singleton, Value};
use crate::singleton_borrow;
use chrono::Utc;

pub struct CounterReset<C: CountersPort> {
  act: CounterAction<C>,
}

impl<C: CountersPort> CounterReset<C> {
  pub fn from_args<X: CountersPort>(
    args: ModuleArgs,
    counters: Singleton<Counters<X>>,
  ) -> CounterReset<X> {
    CounterReset {
      act: CounterAction::<X>::from_args(args, counters, "CounterReset", "graceSeconds"),
    }
  }
}

impl<C: CountersPort> Action for CounterReset<C> {
  fn act(&mut self, record: &mut Record) -> Result<(), Error> {
    let k = &self.act.counter_key;
    match record.get(k) {
      None => Err(format!("Key {} not found in the log entry", k).into()),
      Some(v) => {
        let count = singleton_borrow!(self.act.counters).reset(
          (self.act.counter_name.as_ref(), v),
          self.act.duration.map(|d| Utc::now() + d),
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
  use crate::domain::action::CounterReset;
  use crate::domain::test_util::FakeCountersAdapter;
  use crate::domain::{Action, CounterData, Counters, Singleton, Value};
  use crate::{singleton_borrow, singleton_new, singleton_share};
  use chrono::{Duration, Utc};
  use std::collections::HashMap;

  #[test]
  fn when_reset_without_gracetime_then_count_is_0_and_counter_removed() {
    let (counters, mut action) = get_counters_action(None);
    let mut record = HashMap::with_capacity(1);
    record.insert("k".into(), Value::Str("reset#1".into()));
    singleton_borrow!(counters).insert(("test".into(), Value::Str("reset#1".into())), (5, None));

    action.act(&mut record).unwrap();
    assert_eq!(Some(&Value::Int(0)), record.get("reset"));
    assert_eq!(0, singleton_borrow!(counters).len());
  }

  #[test]
  fn when_reset_with_gracetime_then_count_is_0_and_gracetime_is_stored() {
    let (counters, mut action) = get_counters_action(Some(5));
    let mut record = HashMap::with_capacity(1);
    record.insert("k".into(), Value::Str("reset#2".into()));

    let almost = Utc::now() + Duration::seconds(5);
    let after = almost + Duration::seconds(1);
    action.act(&mut record).unwrap();
    assert_eq!(Some(&Value::Int(0)), record.get("reset"));
    let (c, od) = *(singleton_borrow!(counters)
      .get(&("test".into(), Value::Str("reset#2".into())))
      .unwrap());
    let d = od.unwrap();
    assert!(d >= almost);
    assert!(d < after);
    assert_eq!(0 as usize, c);
  }

  fn get_counters_action(
    grace_time: Option<isize>,
  ) -> (
    Singleton<HashMap<(String, Value), CounterData>>,
    CounterReset<FakeCountersAdapter>,
  ) {
    let counters = singleton_new!(HashMap::new());
    let counters_backend =
      singleton_new!(Counters::<FakeCountersAdapter>::new(FakeCountersAdapter {
        counters: singleton_share!(counters)
      }));
    let mut args = HashMap::with_capacity(grace_time.map(|_| 4).unwrap_or(3));
    args.insert("counter".into(), Value::Str("test".into()));
    args.insert("for".into(), Value::Str("k".into()));
    args.insert("save".into(), Value::Str("reset".into()));
    if let Some(sec) = grace_time {
      args.insert("graceSeconds".into(), Value::Int(sec));
    }
    let action = CounterReset::<FakeCountersAdapter>::from_args(args, counters_backend);
    (counters, action)
  }
}
