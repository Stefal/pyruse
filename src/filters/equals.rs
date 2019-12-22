use crate::modules::{AvailableFilter,Filter,ModuleArgs};
use crate::common::Record;
use crate::common::Value;

#[derive(Debug)]
pub struct Equals {
  field: String,
  value: Value
}

inventory::submit! {
  AvailableFilter::new("filter_equals", move |a| Box::new(Equals::from_args(a)))
}

impl Equals {
  pub fn from_args(mut args: ModuleArgs) -> Equals {
    Equals {
      field: match args.remove("field") {
        Some(Value::Str(s)) => s,
        _ => panic!("The Equals filter needs a field to filter in “field”")
      },
      value: args.remove("value").expect("The Equals filter needs a reference value in “value”")
    }
  }
}

impl Filter for Equals {
  fn filter(&self, record: &mut Record) -> bool {
    match (record.get(&self.field.as_ref()), &self.value) {
      (Some(ref v1), ref v2) => v1 == v2,
      (None, _) => false
    }
  }
}

#[cfg(test)]
mod tests {
  use chrono::Utc;
  use std::collections::HashMap;
  use crate::common::{Record,Value};
  use crate::filters::Equals;
  use crate::modules::{Filter,ModuleArgs};

  fn generate_args_record_equal<'a>(name: &'a str, value: Value) -> (ModuleArgs, Record<'a>) {
    let mut args = HashMap::with_capacity(2);
    args.insert(String::from("field"), Value::Str(String::from(name)));
    args.insert(String::from("value"), value.clone());
    let mut record = HashMap::with_capacity(1);
    record.insert(name, value);
    (args, record)
  }

  fn generate_args_record_custom<'a>(ref_name: &str, ref_value: Value, test_name: &'a str, test_value: Value) -> (ModuleArgs, Record<'a>) {
    let mut args = HashMap::with_capacity(2);
    args.insert(String::from("field"), Value::Str(String::from(ref_name)));
    args.insert(String::from("value"), ref_value);
    let mut record = HashMap::with_capacity(1);
    record.insert(test_name, test_value);
    (args, record)
  }

  #[test]
  fn filter_equals_should_return_true() {
    let (args, mut record) = generate_args_record_equal("a_boolean", Value::Bool(false));
    let filter = Equals::from_args(args);
    assert!(filter.filter(&mut record));

    let (args, mut record) = generate_args_record_equal("a_string", Value::Str(String::from("Hello!")));
    let filter = Equals::from_args(args);
    assert!(filter.filter(&mut record));

    let (args, mut record) = generate_args_record_equal("an_integer", Value::Int(2));
    let filter = Equals::from_args(args);
    assert!(filter.filter(&mut record));

    let (args, mut record) = generate_args_record_equal("a_date", Value::Date(Utc::now()));
    let filter = Equals::from_args(args);
    assert!(filter.filter(&mut record));
  }

  #[test]
  fn filter_equals_should_return_false() {
    let (args, mut record) = generate_args_record_custom("a_boolean", Value::Bool(true), "a_boolean", Value::Bool(false));
    let filter = Equals::from_args(args);
    assert!(! filter.filter(&mut record));

    let (args, mut record) = generate_args_record_custom("a_string", Value::Str(String::from("Hello!")), "a_string", Value::Str(String::from("World!")));
    let filter = Equals::from_args(args);
    assert!(! filter.filter(&mut record));

    let (args, mut record) = generate_args_record_custom("an_integer", Value::Int(2), "an_integer", Value::Int(3));
    let filter = Equals::from_args(args);
    assert!(! filter.filter(&mut record));

    let (args, mut record) = generate_args_record_custom("a_date", Value::Date(Utc::now()), "a_date", Value::Date(Utc::now()));
    let filter = Equals::from_args(args);
    assert!(! filter.filter(&mut record));

    let (args, mut record) = generate_args_record_custom("first_one", Value::Int(1), "second_one", Value::Int(1));
    let filter = Equals::from_args(args);
    assert!(! filter.filter(&mut record));
  }
}
