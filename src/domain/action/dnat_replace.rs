use super::{get_acceptable_key, remove_acceptable_key};
use crate::domain::{
  Action, DnatMapping, DnatMappingsPort, Error, ModuleArgs, Record, Singleton, Value,
};
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
  fn act(&mut self, record: &mut Record) -> Result<(), Error> {
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
    args.insert("addr".into(), Value::Str("int_ip".into()));
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
    args.insert("saddrInto".into(), Value::Str("src_ip".into()));
    let _ = DnatReplace::from_args(args, singleton_share!(mappings));
  }

  #[test]
  fn when_saddrinto_and_at_least_one_match_field_then_no_error() {
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: Vec::new()
    });
    let mut args = HashMap::with_capacity(2);
    args.insert("saddrInto".into(), Value::Str("src_ip".into()));
    args.insert("dport".into(), Value::Int(1234));
    let action = DnatReplace::from_args(args, singleton_share!(mappings));
    assert_eq!(
      vec!(("1234".into(), Some("dp".into()))),
      action
        .matchers
        .iter()
        .map(|(f, g)| (f.clone(), g(&mapping_getter_identification()).clone()))
        .collect::<Vec<(String, Option<String>)>>()
    );
    assert_eq!(
      vec!(("src_ip".into(), Some("sa".into()))),
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
    args.insert("saddrInto".into(), Value::Str("src_ip".into()));
    args.insert("port".into(), Value::Str("src_port".into()));
    let mut record = HashMap::new();
    record.insert("src_ip".into(), Value::Str("prox".into()));
    record.insert("dest_ip".into(), Value::Str("serv".into()));
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
    args.insert("saddrInto".into(), Value::Str("src_ip".into()));
    args.insert("port".into(), Value::Str("src_port".into()));
    let mut record = HashMap::with_capacity(3);
    record.insert("src_ip".into(), Value::Str("prox".into()));
    record.insert("src_port".into(), Value::Str("1234".into()));
    record.insert("dest_ip".into(), Value::Str("serv".into()));
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
    args.insert("saddrInto".into(), Value::Str("src_ip".into()));
    args.insert("port".into(), Value::Str("src_port".into()));
    let mut record = HashMap::with_capacity(3);
    record.insert("src_ip".into(), Value::Str("prox".into()));
    record.insert("src_port".into(), Value::Int(12345));
    record.insert("dest_ip".into(), Value::Str("serv".into()));
    let mut action = DnatReplace::from_args(args, singleton_share!(mappings));
    action.act(&mut record).unwrap();
    assert_eq!(3, record.len());
    assert_eq!(Some(&Value::Str("bad".into())), record.get("src_ip"));
    assert_eq!(Some(&Value::Int(12345)), record.get("src_port"));
    assert_eq!(Some(&Value::Str("serv".into())), record.get("dest_ip"));
  }

  fn mapping_getter_identification() -> DnatMapping {
    DnatMapping {
      src_addr: Some("sa".into()),
      src_port: Some("sp".into()),
      internal_addr: Some("ia".into()),
      internal_port: Some("ip".into()),
      dest_addr: Some("da".into()),
      dest_port: Some("dp".into()),
      keep_until: Utc::now(),
    }
  }

  fn sample_dnat_mapping() -> DnatMapping {
    DnatMapping {
      src_addr: Some("bad".into()),
      src_port: None,
      internal_addr: Some("prox".into()),
      internal_port: Some("12345".into()),
      dest_addr: Some("serv".into()),
      dest_port: None,
      keep_until: Utc::now() + Duration::hours(1),
    }
  }
}
