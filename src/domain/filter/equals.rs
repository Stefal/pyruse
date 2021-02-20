use crate::domain::{Filter, ModuleArgs, Record, Value};

#[derive(Debug)]
pub struct Equals {
  field: String,
  value: Value,
}

impl Equals {
  pub fn from_args(mut args: ModuleArgs) -> Equals {
    Equals {
      field: match args.remove("field") {
        Some(Value::Str(s)) => s,
        _ => panic!("The Equals filter needs a field to filter in “field”"),
      },
      value: args
        .remove("value")
        .expect("The Equals filter needs a reference value in “value”"),
    }
  }
}

impl Filter for Equals {
  fn filter(&self, record: &mut Record) -> bool {
    match (record.get(&self.field), &self.value) {
      (Some(ref v1), ref v2) => v1 == v2,
      (None, _) => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::filter::Equals;
  use crate::domain::{Filter, ModuleArgs, Record, Value};
  use chrono::Utc;
  use std::collections::HashMap;

  fn generate_args_record_equal(name: String, value: Value) -> (ModuleArgs, Record) {
    let mut args = HashMap::with_capacity(2);
    args.insert(String::from("field"), Value::Str(name.clone()));
    args.insert(String::from("value"), value.clone());
    let mut record = HashMap::with_capacity(1);
    record.insert(name, value);
    (args, record)
  }

  fn generate_args_record_custom(
    ref_name: String,
    ref_value: Value,
    test_name: String,
    test_value: Value,
  ) -> (ModuleArgs, Record) {
    let mut args = HashMap::with_capacity(2);
    args.insert(String::from("field"), Value::Str(ref_name));
    args.insert(String::from("value"), ref_value);
    let mut record = HashMap::with_capacity(1);
    record.insert(test_name, test_value);
    (args, record)
  }

  #[test]
  fn filter_equals_returns_true_for_identical_bools() {
    // Given
    let (args, mut record) =
      generate_args_record_equal(String::from("a_boolean"), Value::Bool(false));
    let filter = Equals::from_args(args);

    // Then
    assert!(filter.filter(&mut record));
  }

  #[test]
  fn filter_equals_returns_true_for_identical_strings() {
    // Given
    let (args, mut record) =
      generate_args_record_equal(String::from("a_string"), Value::Str(String::from("Hello!")));
    let filter = Equals::from_args(args);

    // Then
    assert!(filter.filter(&mut record));
  }

  #[test]
  fn filter_equals_returns_true_for_identical_ints() {
    // Given
    let (args, mut record) = generate_args_record_equal(String::from("an_integer"), Value::Int(2));
    let filter = Equals::from_args(args);

    // Then
    assert!(filter.filter(&mut record));
  }

  #[test]
  fn filter_equals_returns_true_for_identical_dates() {
    // Given
    let (args, mut record) =
      generate_args_record_equal(String::from("a_date"), Value::Date(Utc::now()));
    let filter = Equals::from_args(args);

    // Then
    assert!(filter.filter(&mut record));
  }

  #[test]
  fn filter_equals_returns_false_for_different_bools() {
    // Given
    let (args, mut record) = generate_args_record_custom(
      String::from("a_boolean"),
      Value::Bool(true),
      String::from("a_boolean"),
      Value::Bool(false),
    );
    let filter = Equals::from_args(args);

    // Then
    assert!(!filter.filter(&mut record));
  }

  #[test]
  fn filter_equals_returns_false_for_different_strings() {
    // Given
    let (args, mut record) = generate_args_record_custom(
      String::from("a_string"),
      Value::Str(String::from("Hello!")),
      String::from("a_string"),
      Value::Str(String::from("World!")),
    );
    let filter = Equals::from_args(args);

    // Then
    assert!(!filter.filter(&mut record));
  }

  #[test]
  fn filter_equals_returns_false_for_different_ints() {
    // Given
    let (args, mut record) = generate_args_record_custom(
      String::from("an_integer"),
      Value::Int(2),
      String::from("an_integer"),
      Value::Int(3),
    );
    let filter = Equals::from_args(args);

    // Then
    assert!(!filter.filter(&mut record));
  }

  #[test]
  fn filter_equals_returns_false_for_different_dates() {
    // Given
    let (args, mut record) = generate_args_record_custom(
      String::from("a_date"),
      Value::Date(Utc::now()),
      String::from("a_date"),
      Value::Date(Utc::now()),
    );
    let filter = Equals::from_args(args);

    // Then
    assert!(!filter.filter(&mut record));
  }

  #[test]
  fn filter_equals_returns_false_for_non_matching_keys() {
    // Given
    let (args, mut record) = generate_args_record_custom(
      String::from("first_one"),
      Value::Int(1),
      String::from("second_one"),
      Value::Int(1),
    );
    let filter = Equals::from_args(args);

    // Then
    assert!(!filter.filter(&mut record));
  }
}
