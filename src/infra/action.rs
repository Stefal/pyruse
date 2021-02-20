use crate::domain::action::Noop;
use crate::domain::AvailableAction;

const ACTION_NOOP: &str = "action_noop";

inventory::submit! {
  AvailableAction::new(ACTION_NOOP.to_string(), move |a| Box::new(Noop::from_args(a)))
}

#[cfg(test)]
mod tests {
  use crate::domain::{ModuleArgs, ModulesPort};
  use crate::infra::module::InventoryModulesAdapter;
  use std::collections::HashMap;

  #[test]
  fn action_noop_is_available() {
    // Given
    let args: ModuleArgs = HashMap::new();

    // When
    let aa = (InventoryModulesAdapter {}).available_actions();

    // Then
    assert!(aa.contains_key(&super::ACTION_NOOP.to_string()));
    let _can_instantiate = (aa[&super::ACTION_NOOP.to_string()].cons)(args);
  }
}
