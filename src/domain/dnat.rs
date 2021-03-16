use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DnatMapping {
  pub src_addr: Option<String>,
  pub src_port: Option<String>,
  pub internal_addr: Option<String>,
  pub internal_port: Option<String>,
  pub dest_addr: Option<String>,
  pub dest_port: Option<String>,
  pub keep_until: DateTime<Utc>,
}

pub trait DnatMappingsPort {
  fn put(&mut self, mapping: DnatMapping);
  fn get_all(&mut self) -> Vec<&DnatMapping>;
}
