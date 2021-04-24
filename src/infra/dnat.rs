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

#[cfg(test)]
mod tests {
  use crate::domain::{DnatMapping, DnatMappingsPort};
  use crate::infra::dnat::InMemoryDnatMappingsAdapter;
  use chrono::Utc;

  #[test]
  fn keep_until_is_taken_into_account() {
    let already_past = Utc::now();
    let mut mappings = InMemoryDnatMappingsAdapter::new();
    let mapping = DnatMapping {
      src_addr: None,
      src_port: None,
      internal_addr: None,
      internal_port: None,
      dest_addr: None,
      dest_port: None,
      keep_until: already_past,
    };
    mappings.put(mapping);
    assert_eq!(Vec::<&DnatMapping>::new(), mappings.get_all());
  }
}
