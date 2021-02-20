use crate::domain::filter::Equals;
use crate::domain::AvailableFilter;

const FILTER_EQUALS: &str = "filter_equals";

inventory::submit! {
  AvailableFilter::new(FILTER_EQUALS.to_string(), move |a| Box::new(Equals::from_args(a)))
}

#[cfg(test)]
mod tests {
  use crate::domain::{ModuleArgs, ModulesPort, Value};
  use crate::infra::module::InventoryModulesAdapter;
  use std::collections::HashMap;

  #[test]
  fn filter_equals_is_available() {
    // Given
    let mut args: ModuleArgs = HashMap::new();
    args.insert("field".to_string(), Value::Str("a_field".to_string()));
    args.insert("value".to_string(), Value::Int(1));

    // When
    let af = (InventoryModulesAdapter {}).available_filters();

    // Then
    assert!(af.contains_key(&super::FILTER_EQUALS.to_string()));
    let _can_instantiate = (af[&super::FILTER_EQUALS.to_string()].cons)(args);
  }
}
