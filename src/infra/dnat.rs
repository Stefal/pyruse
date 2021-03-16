use crate::domain::{DnatMapping, DnatMappingsPort};
use chrono::Utc;

pub struct InMemoryDnatMappingsAdapter {
  mappings: Vec<DnatMapping>,
}

impl InMemoryDnatMappingsAdapter {
  pub fn new() -> Self {
    InMemoryDnatMappingsAdapter {
      mappings: Vec::new(),
    }
  }
  fn clean(&mut self) {
    let now = Utc::now();
    self.mappings.retain(|m| m.keep_until > now);
  }
}

impl DnatMappingsPort for InMemoryDnatMappingsAdapter {
  fn put(&mut self, mapping: DnatMapping) {
    self.clean();
    self.mappings.push(mapping);
  }
  fn get_all(&mut self) -> Vec<&DnatMapping> {
    self.clean();
    self.mappings.iter().collect()
  }
}
