use super::{get_acceptable_key, remove_acceptable_key};
use crate::domain::{Action, DnatMapping, DnatMappingsPort, ModuleArgs, Record, Singleton, Value};
use crate::singleton_borrow;

type MappingGetter = fn(&DnatMapping) -> &Option<String>;
const SADDR_GETTER: MappingGetter = |m| &m.src_addr;
const SPORT_GETTER: MappingGetter = |m| &m.src_port;
const ADDR_GETTER: MappingGetter = |m| &m.internal_addr;
const PORT_GETTER: MappingGetter = |m| &m.internal_port;
const DADDR_GETTER: MappingGetter = |m| &m.dest_addr;
const DPORT_GETTER: MappingGetter = |m| &m.dest_port;

type FieldAndGetter = (String, MappingGetter);

pub struct DnatReplace {
  mappings: Singleton<dyn DnatMappingsPort>,
  matchers: Vec<FieldAndGetter>,
  updaters: Vec<FieldAndGetter>,
}

impl DnatReplace {
  pub fn from_args(mut args: ModuleArgs, mappings: Singleton<dyn DnatMappingsPort>) -> DnatReplace {
    let mut matchers = Vec::new();
    let mut updaters = Vec::new();
    if let Some(s) = remove_acceptable_key(&mut args, "addr") {
      matchers.push((s, ADDR_GETTER));
    }
    if let Some(s) = remove_acceptable_key(&mut args, "port") {
      matchers.push((s, PORT_GETTER));
    }
    if let Some(s) = remove_acceptable_key(&mut args, "daddr") {
      matchers.push((s, DADDR_GETTER));
    }
    if let Some(s) = remove_acceptable_key(&mut args, "dport") {
      matchers.push((s, DPORT_GETTER));
    }
    if matchers.is_empty() {
      panic!("The DnatReplace action needs at least one log field on which to do the matching");
    }
    updaters.push((
      remove_acceptable_key(&mut args, "saddrInto")
        .expect("The DnatReplace action needs a log field to replace in “saddrInto”"),
      SADDR_GETTER,
    ));
    if let Some(s) = remove_acceptable_key(&mut args, "sportInto") {
      updaters.push((s, SPORT_GETTER));
    }
    DnatReplace {
      mappings,
      matchers,
      updaters,
    }
  }
}

impl Action for DnatReplace {
  fn act(&mut self, record: &mut Record) -> Result<(), ()> {
    for (field, _) in self.matchers.iter() {
      if !record.contains_key(field) {
        return Ok(()); // not applicable
      }
    }
    for mapping in singleton_borrow!(self.mappings).get_all().iter() {
      let mut found = true;
      for (field, getter) in self.matchers.iter() {
        if &get_acceptable_key(record, field) != getter(*mapping) {
          found = false; // not matching
          break;
        }
      }
      if found {
        for (field, getter) in self.updaters.iter() {
          if let Some(s) = getter(mapping) {
            record.insert(field.clone(), Value::Str(s.clone()));
          }
        }
        return Ok(()); // replacement done; stop here
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::DnatReplace;
  use crate::domain::test_util::FakeDnatMappings;
  use crate::domain::{Action, DnatMapping, Value};
  use crate::{singleton_new, singleton_share};
  use chrono::{Duration, Utc};
  use std::collections::HashMap;

  #[test]
  #[should_panic(expected = "The DnatReplace action needs a log field to replace in “saddrInto”")]
  fn when_no_saddrinto_then_error() {
    let mut args = HashMap::with_capacity(1);
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: Vec::new()
    });
    args.insert("addr".to_string(), Value::Str("int_ip".to_string()));
    let _ = DnatReplace::from_args(args, singleton_share!(mappings));
  }

  #[test]
  #[should_panic(
    expected = "The DnatReplace action needs at least one log field on which to do the matching"
  )]
  fn when_no_match_field_then_error() {
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: Vec::new()
    });
    let mut args = HashMap::with_capacity(1);
    args.insert("saddrInto".to_string(), Value::Str("src_ip".to_string()));
    let _ = DnatReplace::from_args(args, singleton_share!(mappings));
  }

  #[test]
  fn when_saddrinto_and_at_least_one_match_field_then_no_error() {
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: Vec::new()
    });
    let mut args = HashMap::with_capacity(2);
    args.insert("saddrInto".to_string(), Value::Str("src_ip".to_string()));
    args.insert("dport".to_string(), Value::Int(1234));
    let action = DnatReplace::from_args(args, singleton_share!(mappings));
    assert_eq!(
      vec!(("1234".to_string(), Some("dp".to_string()))),
      action
        .matchers
        .iter()
        .map(|(f, g)| (f.clone(), g(&mapping_getter_identification()).clone()))
        .collect::<Vec<(String, Option<String>)>>()
    );
    assert_eq!(
      vec!(("src_ip".to_string(), Some("sa".to_string()))),
      action
        .updaters
        .iter()
        .map(|(f, g)| (f.clone(), g(&mapping_getter_identification()).clone()))
        .collect::<Vec<(String, Option<String>)>>()
    );
  }

  #[test]
  fn when_no_matching_entry_then_no_change() {
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: vec!(sample_dnat_mapping()),
    });
    let mut args = HashMap::with_capacity(2);
    args.insert("saddrInto".to_string(), Value::Str("src_ip".to_string()));
    args.insert("port".to_string(), Value::Str("src_port".to_string()));
    let mut record = HashMap::new();
    record.insert("src_ip".to_string(), Value::Str("prox".to_string()));
    record.insert("dest_ip".to_string(), Value::Str("serv".to_string()));
    let expected = record.clone();
    let mut action = DnatReplace::from_args(args, singleton_share!(mappings));
    action.act(&mut record).unwrap();
    assert_eq!(expected, record);
  }

  #[test]
  fn when_no_matching_value_then_no_change() {
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: vec!(sample_dnat_mapping()),
    });
    let mut args = HashMap::with_capacity(2);
    args.insert("saddrInto".to_string(), Value::Str("src_ip".to_string()));
    args.insert("port".to_string(), Value::Str("src_port".to_string()));
    let mut record = HashMap::with_capacity(3);
    record.insert("src_ip".to_string(), Value::Str("prox".to_string()));
    record.insert("src_port".to_string(), Value::Str("1234".to_string()));
    record.insert("dest_ip".to_string(), Value::Str("serv".to_string()));
    let expected = record.clone();
    let mut action = DnatReplace::from_args(args, singleton_share!(mappings));
    action.act(&mut record).unwrap();
    assert_eq!(expected, record);
  }

  #[test]
  fn when_matching_entry_then_change() {
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: vec!(sample_dnat_mapping()),
    });
    let mut args = HashMap::with_capacity(2);
    args.insert("saddrInto".to_string(), Value::Str("src_ip".to_string()));
    args.insert("port".to_string(), Value::Str("src_port".to_string()));
    let mut record = HashMap::with_capacity(3);
    record.insert("src_ip".to_string(), Value::Str("prox".to_string()));
    record.insert("src_port".to_string(), Value::Int(12345));
    record.insert("dest_ip".to_string(), Value::Str("serv".to_string()));
    let mut action = DnatReplace::from_args(args, singleton_share!(mappings));
    action.act(&mut record).unwrap();
    assert_eq!(3, record.len());
    assert_eq!(Some(&Value::Str("bad".to_string())), record.get("src_ip"));
    assert_eq!(Some(&Value::Int(12345)), record.get("src_port"));
    assert_eq!(Some(&Value::Str("serv".to_string())), record.get("dest_ip"));
  }

  fn mapping_getter_identification() -> DnatMapping {
    DnatMapping {
      src_addr: Some("sa".to_string()),
      src_port: Some("sp".to_string()),
      internal_addr: Some("ia".to_string()),
      internal_port: Some("ip".to_string()),
      dest_addr: Some("da".to_string()),
      dest_port: Some("dp".to_string()),
      keep_until: Utc::now(),
    }
  }

  fn sample_dnat_mapping() -> DnatMapping {
    DnatMapping {
      src_addr: Some("bad".to_string()),
      src_port: None,
      internal_addr: Some("prox".to_string()),
      internal_port: Some("12345".to_string()),
      dest_addr: Some("serv".to_string()),
      dest_port: None,
      keep_until: Utc::now() + Duration::hours(1),
    }
  }
}
