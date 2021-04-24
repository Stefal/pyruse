use crate::domain::Value;
use chrono::{DateTime, Utc};

pub type CounterRef<'t> = (&'t str, &'t Value);
pub type CounterData = (usize, Option<DateTime<Utc>>);

pub trait CountersPort {
  fn modify(
    &mut self,
    entry: CounterRef,
    data: CounterData,
    f: impl FnMut(&mut CounterData, CounterData) -> usize,
  ) -> usize;
  fn remove(&mut self, entry: CounterRef) -> Option<CounterData>;
  fn remove_if(&mut self, predicate: impl Fn(&CounterData) -> bool);
}

pub struct Counters<P: CountersPort> {
  backend: P,
}

impl<P: CountersPort> Counters<P> {
  pub fn new<X: CountersPort>(backend: X) -> Counters<X> {
    Counters { backend }
  }

  fn grace_active(data: &CounterData) -> bool {
    if let Some(dt) = data.1 {
      data.0 == 0 && dt > Utc::now()
    } else {
      false
    }
  }

  fn clean(&mut self) {
    let now = Utc::now();
    self.backend.remove_if(|c_data| {
      if let (_, Some(dt)) = c_data {
        let ref_now = &now;
        return dt <= ref_now;
      }
      false
    });
  }

  pub fn set(&mut self, entry: CounterRef, data: CounterData) -> usize {
    self.clean();
    self.backend.modify(entry, data, |value, data| {
      *value = data;
      (*value).0
    })
  }

  pub fn augment(&mut self, entry: CounterRef, data: CounterData) -> usize {
    self.clean();
    self.backend.modify(entry, data, |value, data| {
      if !Counters::<P>::grace_active(&value) {
        (*value).0 = (*value).0 + data.0;
        if let Some(wanted_dt) = data.1 {
          match value.1 {
            Some(existing_dt) if existing_dt < wanted_dt => value.1 = data.1,
            None => value.1 = data.1,
            _ => (),
          }
        }
      }
      (*value).0
    })
  }

  pub fn reset(&mut self, entry: CounterRef, grace_until: Option<DateTime<Utc>>) -> usize {
    self.clean();
    match grace_until {
      Some(_) => {
        // a grace-time is wanted, so the entry must exist…
        self.backend.modify(entry, (0, grace_until), |value, data| {
          match value {
            // … and its grace-time is set to the farther value between existing and requested
            (0, Some(existing_dt)) if *existing_dt > data.1.unwrap() => (),
            _ => (*value) = data,
          };
          0
        })
      }
      None => {
        // no grace-time wanted, so the entry is deleted…
        if let Some((0, Some(existing_dt))) = self.backend.remove(entry) {
          // … unless an existing grace-time was found
          self
            .backend
            .modify(entry, (0, Some(existing_dt)), |value, data| {
              *value = data;
              0
            })
        } else {
          0
        }
      }
    }
  }
}

#[macro_use]
#[cfg(test)]
mod tests {
  use crate::domain::test_util::FakeCountersAdapter;
  use crate::domain::{CounterData, Counters, Singleton, Value};
  use crate::{singleton_borrow, singleton_new, singleton_share};
  use chrono::{Duration, Utc};
  use std::collections::HashMap;
  use std::{thread, time};

  #[test]
  fn set_forces_the_value_of_a_counter() {
    let (counters_store, mut counters) = get_store_counters();
    let (c_ref, stored_key) = get_ref_and_key("test", &Value::Int(5));
    let value = counters.set(c_ref, (9, None));
    assert_eq!(value, 9);
    let stored_value = singleton_borrow!(counters_store)
      .get_mut(&stored_key)
      .unwrap()
      .0;
    assert_eq!(stored_value, 9);
  }

  #[test]
  fn a_counter_starts_from_0() {
    let (counters_store, mut counters) = get_store_counters();
    let (_, stored_key) = get_ref_and_key("test", &Value::Bool(true));
    let stored_value = singleton_borrow!(counters_store)
      .get_mut(&stored_key)
      .map(|_| 0);
    assert_eq!(stored_value, None);
    let value = counters.augment(("test", &Value::Bool(true)), (1, None));
    assert_eq!(value, 1);
    let stored_value = singleton_borrow!(counters_store)
      .get_mut(&stored_key)
      .unwrap()
      .0;
    assert_eq!(stored_value, 1);
  }

  #[test]
  fn augment_raises_a_counter_by_its_amount() {
    let (_, mut counters) = get_store_counters();
    let str_value = Value::Str("string".into());
    counters.set(("test", &str_value), (4, None));
    let value = counters.augment(("test", &str_value), (3, None));
    assert_eq!(value, 7);
  }

  #[test]
  fn reset_without_gracetime_removes_a_counter() {
    let (counters_store, mut counters) = get_store_counters();
    let now = Utc::now();
    let date_value = Value::Date(now.clone());
    let (c_ref, stored_key) = get_ref_and_key("test", &date_value);
    counters.augment(c_ref, (5, None));
    let stored_value = singleton_borrow!(counters_store)
      .get_mut(&stored_key)
      .unwrap()
      .0;
    assert_eq!(stored_value, 5);
    let value = counters.reset(("test", &Value::Date(now)), None);
    assert_eq!(value, 0);
    let stored_value = singleton_borrow!(counters_store)
      .get_mut(&stored_key)
      .map(|_| 0);
    assert_eq!(stored_value, None);
  }

  #[test]
  fn augment_records_the_longest_datetime() {
    let (counters_store, mut counters) = get_store_counters();
    let (c_ref, stored_key) = get_ref_and_key("test", &Value::Bool(true));
    let old_dt = Utc::now() + Duration::minutes(1);
    let new_dt = Utc::now() + Duration::minutes(1);
    assert!(old_dt < new_dt);
    counters.augment(c_ref.clone(), (1, Some(new_dt)));
    counters.augment(c_ref, (3, Some(old_dt)));
    let stored_dt = singleton_borrow!(counters_store)
      .get_mut(&stored_key)
      .unwrap()
      .1
      .unwrap();
    assert_eq!(stored_dt, new_dt);
  }

  #[test]
  fn augment_without_timeout_is_ignored_in_the_presence_of_a_gracetime() {
    let (counters_store, mut counters) = get_store_counters();
    let (c_ref, stored_key) = get_ref_and_key("test", &Value::Bool(true));
    let future_dt = Utc::now() + Duration::days(1);
    counters.reset(c_ref.clone(), Some(future_dt));
    let value = counters.augment(c_ref, (3, None));
    assert_eq!(value, 0);
    assert_eq!(
      &(0 as usize, Some(future_dt)),
      singleton_borrow!(counters_store)
        .get_mut(&stored_key)
        .unwrap()
    );
  }

  #[test]
  fn augment_with_timeout_is_ignored_in_the_presence_of_a_gracetime() {
    let (counters_store, mut counters) = get_store_counters();
    let (c_ref, stored_key) = get_ref_and_key("test", &Value::Bool(true));
    let future_dt = Utc::now() + Duration::days(1);
    let soon_dt = Utc::now() + Duration::hours(1);
    counters.reset(c_ref.clone(), Some(future_dt));
    let value = counters.augment(c_ref, (3, Some(soon_dt)));
    assert_eq!(value, 0);
    assert_eq!(
      &(0 as usize, Some(future_dt)),
      singleton_borrow!(counters_store)
        .get_mut(&stored_key)
        .unwrap()
    );
  }

  #[test]
  fn augment_also_cleans_obsolete_counters() {
    let (counters_store, mut counters) = get_store_counters();
    let (c_ref, stored_key) = get_ref_and_key("test", &Value::Bool(true));
    let future_dt = Utc::now() + Duration::milliseconds(500);
    counters.augment(c_ref, (3, Some(future_dt)));
    assert_eq!(1, singleton_borrow!(counters_store).len());
    assert_eq!(
      3,
      singleton_borrow!(counters_store)
        .get_mut(&stored_key)
        .unwrap()
        .0
    );
    thread::sleep(time::Duration::from_secs(1));
    let (c_ref, stored_key) = get_ref_and_key("test2", &Value::Bool(false));
    let value = counters.augment(c_ref, (5, None));
    assert_eq!(value, 5);
    assert_eq!(1, singleton_borrow!(counters_store).len());
    assert_eq!(
      5,
      singleton_borrow!(counters_store)
        .get_mut(&stored_key)
        .unwrap()
        .0
    );
  }

  fn get_store_counters() -> (
    Singleton<HashMap<(String, Value), CounterData>>,
    Counters<FakeCountersAdapter>,
  ) {
    let counters_store = singleton_new!(HashMap::new());
    let counters = Counters::<FakeCountersAdapter>::new(FakeCountersAdapter {
      counters: singleton_share!(&counters_store),
    });
    (counters_store, counters)
  }

  fn get_ref_and_key<'t>(s: &'t str, v: &'t Value) -> ((&'t str, &'t Value), (String, Value)) {
    let storage_key = (s.to_string(), v.clone());
    let key_ref = (s, v);
    (key_ref, storage_key)
  }
}
