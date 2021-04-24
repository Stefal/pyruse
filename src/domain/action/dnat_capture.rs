use super::{get_acceptable_key, remove_acceptable_key};
use crate::domain::{
  Action, DnatMapping, DnatMappingsPort, Error, ModuleArgs, Record, Singleton, Value,
};
use crate::singleton_borrow;
use chrono::{Duration, Utc};

type FieldAndValue = (Option<String>, Option<String>);

struct DnatMappingSpec {
  pub src_addr: FieldAndValue,
  pub src_port: FieldAndValue,
  pub internal_addr: FieldAndValue,
  pub internal_port: FieldAndValue,
  pub dest_addr: FieldAndValue,
  pub dest_port: FieldAndValue,
  pub keep_duration: Duration,
}

pub struct DnatCapture {
  mappings: Singleton<dyn DnatMappingsPort>,
  specs: DnatMappingSpec,
}

impl DnatCapture {
  pub fn from_args(mut args: ModuleArgs, mappings: Singleton<dyn DnatMappingsPort>) -> DnatCapture {
    let src_addr = (
      Some(
        remove_acceptable_key(&mut args, "saddr")
          .expect("The DnatCapture action needs a log field for the source address in “saddr”"),
      ),
      None,
    );
    let src_port = (remove_acceptable_key(&mut args, "sport"), None);
    let internal_addr = (
      remove_acceptable_key(&mut args, "addr"),
      remove_acceptable_key(&mut args, "addrValue"),
    );
    if let &(None, None) = &internal_addr {
      panic!("The DnatCapture action requires either a field (“addr”) or a value (“addrValue”) for the internal address");
    }
    let internal_port = (
      remove_acceptable_key(&mut args, "port"),
      remove_acceptable_key(&mut args, "portValue"),
    );
    let dest_addr = (
      remove_acceptable_key(&mut args, "daddr"),
      remove_acceptable_key(&mut args, "daddrValue"),
    );
    let dest_port = (
      remove_acceptable_key(&mut args, "dport"),
      remove_acceptable_key(&mut args, "dportValue"),
    );
    let keep_duration = match args.remove("keepSeconds") {
      Some(Value::Int(i)) => Duration::seconds(i as i64),
      _ => Duration::seconds(63),
    };
    DnatCapture {
      mappings,
      specs: DnatMappingSpec {
        src_addr,
        src_port,
        internal_addr,
        internal_port,
        dest_addr,
        dest_port,
        keep_duration,
      },
    }
  }
}

impl Action for DnatCapture {
  fn act(&mut self, record: &mut Record) -> Result<(), Error> {
    let src_addr = value_for(&self.specs.src_addr, record);
    let internal_addr = value_for(&self.specs.internal_addr, record);
    if src_addr == None || internal_addr == None {
      return Ok(());
    }
    let src_port = value_for(&self.specs.src_port, record);
    let internal_port = value_for(&self.specs.internal_port, record);
    let dest_addr = value_for(&self.specs.dest_addr, record);
    let dest_port = value_for(&self.specs.dest_port, record);
    singleton_borrow!(self.mappings).put(DnatMapping {
      src_addr,
      src_port,
      internal_addr,
      internal_port,
      dest_addr,
      dest_port,
      keep_until: Utc::now() + self.specs.keep_duration,
    });
    Ok(())
  }
}

fn value_for(spec: &FieldAndValue, record: &Record) -> Option<String> {
  (&spec.0)
    .as_deref()
    .and_then(|s| get_acceptable_key(record, s))
    .or(spec.1.clone())
}

#[cfg(test)]
mod tests {
  use super::DnatCapture;
  use crate::domain::test_util::FakeDnatMappings;
  use crate::domain::{Action, DnatMapping, DnatMappingsPort, ModuleArgs, Record, Value};
  use crate::{singleton_borrow, singleton_new, singleton_share};
  use chrono::{Duration, Utc};
  use std::collections::HashMap;

  #[test]
  #[should_panic(
    expected = "The DnatCapture action needs a log field for the source address in “saddr”"
  )]
  fn when_no_saddr_then_error() {
    let mut args = HashMap::with_capacity(1);
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: Vec::new()
    });
    args.insert("addr".into(), Value::Str("int_ip".into()));
    let _ = DnatCapture::from_args(args, singleton_share!(mappings));
  }

  #[test]
  #[should_panic(
    expected = "The DnatCapture action requires either a field (“addr”) or a value (“addrValue”) for the internal address"
  )]
  fn when_no_addr_nor_addr_value_then_error() {
    let mut args = HashMap::with_capacity(1);
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: Vec::new()
    });
    args.insert("saddr".into(), Value::Str("src_ip".into()));
    let _ = DnatCapture::from_args(args, singleton_share!(mappings));
  }

  #[test]
  fn when_no_addr_but_addr_value_then_no_error() {
    let mut args = HashMap::with_capacity(2);
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: Vec::new()
    });
    args.insert("saddr".into(), Value::Str("src_ip".into()));
    args.insert("addrValue".into(), Value::Str("1.2.3.4".into()));
    let _ = DnatCapture::from_args(args, singleton_share!(mappings));
    assert!(true);
  }

  #[test]
  fn when_no_addr_value_but_addr_then_no_error() {
    let mut args = HashMap::with_capacity(2);
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: Vec::new()
    });
    args.insert("saddr".into(), Value::Str("src_ip".into()));
    args.insert("addr".into(), Value::Str("int_ip".into()));
    let _ = DnatCapture::from_args(args, singleton_share!(mappings));
    assert!(true);
  }

  #[test]
  fn when_no_keep_seconds_then_63sec() {
    let mut args = HashMap::with_capacity(2);
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: Vec::new()
    });
    args.insert("saddr".into(), Value::Str("src_ip".into()));
    args.insert("addr".into(), Value::Str("int_ip".into()));
    let action = DnatCapture::from_args(args, singleton_share!(mappings));
    assert_eq!(Duration::seconds(63), action.specs.keep_duration);
  }

  #[test]
  fn when_insufficient_entry_then_no_mapping() {
    let mut args = HashMap::with_capacity(2);
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: Vec::new()
    });
    args.insert("saddr".into(), Value::Str("src_ip".into()));
    args.insert("addr".into(), Value::Str("int_ip".into()));
    let mut action = DnatCapture::from_args(args, singleton_share!(mappings));
    action.act(&mut HashMap::new()).unwrap();
    assert_eq!(0, singleton_borrow!(mappings).mappings.len());
  }

  fn when_field_and_or_value_then_check_mapping(
    mut args: ModuleArgs,
    entry_with_addr: bool,
    entry_with_daddr: bool,
    expect: DnatMapping,
  ) {
    let mappings = singleton_new!(FakeDnatMappings {
      mappings: Vec::new()
    });

    // specify the Action
    args.insert("saddr".into(), Value::Str("sa".into()));

    // prepare the entry
    let mut entry: Record = HashMap::with_capacity(6);
    entry.insert("sa".into(), Value::Str("vsa".into()));
    entry.insert("sp".into(), Value::Str("vsp".into()));
    if entry_with_addr {
      entry.insert("a".into(), Value::Str("va".into()));
      entry.insert("p".into(), Value::Str("vp".into()));
    }
    if entry_with_daddr {
      entry.insert("da".into(), Value::Str("vda".into()));
      entry.insert("dp".into(), Value::Str("vdp".into()));
    }

    // run
    let mut action = DnatCapture::from_args(args, singleton_share!(mappings));
    action.act(&mut entry).unwrap();

    // check the result
    assert_eq!(1, singleton_borrow!(mappings).get_all().len());
    let got = singleton_borrow!(mappings)
      .get_all()
      .last()
      .map(|m| {
        let mut m = (**m).clone();
        m.keep_until = expect.keep_until;
        m
      })
      .unwrap();
    assert_eq!(expect, got);
  }

  #[test]
  fn when_sufficient_record_a_mapping_is_stored() {
    when_field_and_or_value_then_check_mapping(
      as_args(vec![("addr", "a")]),
      true,
      true,
      test_dnat_mapping(None, Some("va"), None, None, None),
    );
    when_field_and_or_value_then_check_mapping(
      as_args(vec![("addrValue", "x")]),
      true,
      true,
      test_dnat_mapping(None, Some("x"), None, None, None),
    );
    when_field_and_or_value_then_check_mapping(
      as_args(vec![("addr", "a"), ("addrValue", "x")]),
      true,
      true,
      test_dnat_mapping(None, Some("va"), None, None, None),
    );
    when_field_and_or_value_then_check_mapping(
      as_args(vec![("addr", "a"), ("addrValue", "x")]),
      false,
      true,
      test_dnat_mapping(None, Some("x"), None, None, None),
    );

    when_field_and_or_value_then_check_mapping(
      as_args(vec![("addr", "a"), ("daddr", "da")]),
      true,
      true,
      test_dnat_mapping(None, Some("va"), None, Some("vda"), None),
    );
    when_field_and_or_value_then_check_mapping(
      as_args(vec![("addr", "a"), ("daddrValue", "x")]),
      true,
      true,
      test_dnat_mapping(None, Some("va"), None, Some("x"), None),
    );
    when_field_and_or_value_then_check_mapping(
      as_args(vec![("addr", "a"), ("daddr", "da"), ("daddrValue", "x")]),
      true,
      true,
      test_dnat_mapping(None, Some("va"), None, Some("vda"), None),
    );
    when_field_and_or_value_then_check_mapping(
      as_args(vec![("addr", "a"), ("daddr", "da"), ("daddrValue", "x")]),
      true,
      false,
      test_dnat_mapping(None, Some("va"), None, Some("x"), None),
    );

    when_field_and_or_value_then_check_mapping(
      as_args(vec![("addr", "a"), ("port", "p")]),
      true,
      true,
      test_dnat_mapping(None, Some("va"), Some("vp"), None, None),
    );
    when_field_and_or_value_then_check_mapping(
      as_args(vec![("addr", "a"), ("dport", "dp")]),
      true,
      true,
      test_dnat_mapping(None, Some("va"), None, None, Some("vdp")),
    );
  }

  fn test_dnat_mapping(
    src_port: Option<&str>,
    internal_addr: Option<&str>,
    internal_port: Option<&str>,
    dest_addr: Option<&str>,
    dest_port: Option<&str>,
  ) -> DnatMapping {
    DnatMapping {
      src_addr: Some("vsa".into()),
      src_port: src_port.map(|s| s.to_string()),
      internal_addr: internal_addr.map(|s| s.to_string()),
      internal_port: internal_port.map(|s| s.to_string()),
      dest_addr: dest_addr.map(|s| s.to_string()),
      dest_port: dest_port.map(|s| s.to_string()),
      keep_until: Utc::now(),
    }
  }
  fn as_args(data: Vec<(&str, &str)>) -> ModuleArgs {
    let mut args = HashMap::with_capacity(data.len());
    data.iter().for_each(|(f, v)| {
      args.insert(f.to_string(), Value::Str(v.to_string()));
    });
    args
  }
}
