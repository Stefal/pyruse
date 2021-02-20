use crate::domain::{AvailableAction, AvailableFilter, ModulesPort};
use std::collections::HashMap;

inventory::collect!(AvailableAction);
inventory::collect!(AvailableFilter);

pub struct InventoryModulesAdapter {}

impl ModulesPort for InventoryModulesAdapter {
  fn available_actions(&self) -> HashMap<&String, &AvailableAction> {
    let mut h: HashMap<&String, &AvailableAction> = HashMap::new();
    for action in inventory::iter::<AvailableAction> {
      h.insert(&action.name, &action);
    }
    h
  }

  fn available_filters(&self) -> HashMap<&String, &AvailableFilter> {
    let mut h: HashMap<&String, &AvailableFilter> = HashMap::new();
    for filter in inventory::iter::<AvailableFilter> {
      h.insert(&filter.name, &filter);
    }
    h
  }
}
