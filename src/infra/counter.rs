use crate::domain::{CounterData, CounterRef, CountersPort, Value};
use std::collections::HashMap;

type CounterKeys = HashMap<Value, CounterData>;
pub struct InMemoryCounterAdapter {
  counters: HashMap<String, CounterKeys>,
}

impl InMemoryCounterAdapter {
  pub fn new() -> Self {
    InMemoryCounterAdapter {
      counters: HashMap::new(),
    }
  }
}

impl CountersPort for InMemoryCounterAdapter {
  fn modify(
    &mut self,
    entry: CounterRef,
    data: CounterData,
    mut f: impl FnMut(&mut CounterData, CounterData) -> usize,
  ) -> usize {
    if !self.counters.contains_key(entry.0) {
      self.counters.insert(entry.0.to_string(), HashMap::new());
    }
    let keys = self.counters.get_mut(entry.0).unwrap();
    if !keys.contains_key(entry.1) {
      keys.insert(entry.1.clone(), (0, None));
    }
    f(keys.get_mut(entry.1).unwrap(), data)
  }

  fn remove(&mut self, entry: CounterRef) -> Option<CounterData> {
    let (to_remove, option) = match self.counters.get_mut(entry.0) {
      None => (false, None),
      Some(keys) => match keys.remove(entry.1) {
        None => (false, None),
        Some(d) => (keys.is_empty(), Some(d)),
      },
    };
    if to_remove {
      self.counters.remove(entry.0);
    }
    option
  }

  fn remove_if(&mut self, predicate: impl Fn(&CounterData) -> bool) {
    self.counters.retain(|_, name| {
      name.retain(|_, data| !predicate(data));
      !name.is_empty()
    });
  }
}

#[cfg(test)]
mod tests {
  use super::{CounterKeys, InMemoryCounterAdapter};
  use crate::domain::{CountersPort, Value};
  use chrono::Utc;
  use std::collections::HashMap;

  #[test]
  fn modify_allows_modifying_an_entry_and_returns_the_new_value() {
    let mut counters: HashMap<String, CounterKeys> = HashMap::new();
    let counter = "counter";
    let key = Value::Str("1.2.3.4".into());
    let new_data = (2, None);
    counters.insert(counter.to_string(), HashMap::new());
    counters
      .get_mut(counter)
      .unwrap()
      .insert(key.clone(), (1, Some(Utc::now())));
    let mut adapter = InMemoryCounterAdapter { counters };
    let new_value = adapter.modify((counter, &key), new_data.clone(), |existing, new| {
      *existing = new;
      (*existing).0
    });
    assert_eq!(2, new_value);
    assert_eq!(
      &(2 as usize, None),
      adapter.counters.get(counter).unwrap().get(&key).unwrap()
    );
  }

  #[test]
  fn after_remove_the_entry_is_not_there() {
    let mut counters: HashMap<String, CounterKeys> = HashMap::new();
    let counter = "counter";
    let key1 = Value::Str("1.2.3.4".into());
    let key2 = Value::Bool(true);
    let data1 = (2, None);
    let data2 = (5, None);
    counters.insert(counter.to_string(), HashMap::new());
    let map = counters.get_mut(counter).unwrap();
    map.insert(key1.clone(), data1);
    map.insert(key2.clone(), data2);
    let mut adapter = InMemoryCounterAdapter { counters };
    let removed = adapter.remove((counter, &key1));
    assert_eq!(Some(data1), removed);
    assert!(!adapter.counters.get(counter).unwrap().contains_key(&key1));
    assert_eq!(
      &data2,
      adapter.counters.get(counter).unwrap().get(&key2).unwrap()
    );
  }

  #[test]
  fn remove_on_unexisting_entry_does_nothing_and_returns_none() {
    let mut counters: HashMap<String, CounterKeys> = HashMap::new();
    let counter = "counter";
    let key1 = Value::Str("1.2.3.4".into());
    let key2 = Value::Bool(true);
    let data1 = (2, None);
    counters.insert(counter.to_string(), HashMap::new());
    let map = counters.get_mut(counter).unwrap();
    map.insert(key1.clone(), data1);
    let mut adapter = InMemoryCounterAdapter { counters };
    let removed = adapter.remove((counter, &key2));
    assert_eq!(None, removed);
    assert!(adapter.counters.get(counter).unwrap().contains_key(&key1));
    assert_eq!(
      &data1,
      adapter.counters.get(counter).unwrap().get(&key1).unwrap()
    );
  }

  #[test]
  fn after_last_key_is_removed_by_remove_counter_is_also_removed() {
    let mut counters: HashMap<String, CounterKeys> = HashMap::new();
    let counter = "counter";
    let key1 = Value::Str("1.2.3.4".into());
    let key2 = Value::Bool(true);
    let data1 = (2, None);
    let data2 = (5, None);
    counters.insert(counter.to_string(), HashMap::new());
    let map = counters.get_mut(counter).unwrap();
    map.insert(key1.clone(), data1);
    map.insert(key2.clone(), data2);
    let mut adapter = InMemoryCounterAdapter { counters };
    assert_eq!(Some(data1), adapter.remove((counter, &key1)));
    assert_eq!(Some(data2), adapter.remove((counter, &key2)));
    assert!(!adapter.counters.contains_key(counter));
  }

  #[test]
  fn removeif_removes_entries_that_match_the_predicate() {
    let mut counters: HashMap<String, CounterKeys> = HashMap::new();
    let counter = "counter";
    let key1 = Value::Str("1.2.3.4".into());
    let key2 = Value::Bool(true);
    let data1 = (2, None);
    let data2 = (5, None);
    counters.insert(counter.to_string(), HashMap::new());
    let map = counters.get_mut(counter).unwrap();
    map.insert(key1.clone(), data1);
    map.insert(key2.clone(), data2);
    let mut adapter = InMemoryCounterAdapter { counters };
    adapter.remove_if(|(u, _)| *u == 2);
    assert_eq!(1, adapter.counters.get(counter).unwrap().len());
    assert_eq!(
      Some(&(5 as usize, None)),
      adapter.counters.get(counter).unwrap().get(&key2)
    );
  }

  #[test]
  fn after_last_key_is_removed_by_removeif_counter_is_also_removed() {
    let mut counters: HashMap<String, CounterKeys> = HashMap::new();
    let counter = "counter";
    let key1 = Value::Str("1.2.3.4".into());
    let data1 = (2, None);
    counters.insert(counter.to_string(), HashMap::new());
    let map = counters.get_mut(counter).unwrap();
    map.insert(key1.clone(), data1);
    let mut adapter = InMemoryCounterAdapter { counters };
    adapter.remove_if(|(u, _)| *u == 2);
    assert_eq!(0, adapter.counters.len());
  }
}
