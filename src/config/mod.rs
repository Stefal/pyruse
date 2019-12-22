use serde::de::{self,Deserializer,MapAccess,SeqAccess,Visitor};
use serde::Deserialize;
use serde_json;
use serde_yaml;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::io::Read;
use crate::common::Value;
use crate::modules::ModuleArgs;

mod file;

thread_local!(static CONFIG: RefCell<Option<Config>> = RefCell::new(None));

#[derive(Debug,Deserialize)]
pub struct Config {
  actions: HashMap<String, Chain>,

  #[serde(flatten)]
  options: HashMap<String, Value>
}

type Chain = Vec<Step>;

#[derive(Debug,Deserialize)]
pub enum StepType {
  #[serde(rename(deserialize = "action"))]
  Action(String),
  #[serde(rename(deserialize = "filter"))]
  Filter(String)
}

#[derive(Debug,Deserialize)]
pub struct Step {
  #[serde(flatten)]
  module: StepType,
  args: ModuleArgs,
  #[serde(rename(deserialize = "then"))]
  then_dest: Option<String>,
  #[serde(rename(deserialize = "else"))]
  else_dest: Option<String>
}

/* *** serde for Value *** */

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
  type Value = Value;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("a boolean, string, or integer")
  }

  fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Bool(v))
  }

  fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Int(v as isize))
  }

  fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Int(v as isize))
  }

  fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Int(v as isize))
  }

  fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Int(v as isize))
  }

  fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Int(v as isize))
  }

  fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Int(v as isize))
  }

  fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Int(v as isize))
  }

  fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Int(v as isize))
  }

  fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Int(v as isize))
  }

  fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Int(v as isize))
  }

  fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Int(v as isize))
  }

  fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Int(v as isize))
  }

  fn visit_char<E>(self, v: char) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Str(v.to_string()))
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Str(String::from(v)))
  }

  fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Str(String::from(v)))
  }

  fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Str(v))
  }

  fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Str(std::str::from_utf8(v).expect("Strings in the configuration must be UTF-8").to_string()))
  }

  fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Str(std::str::from_utf8(v).expect("Strings in the configuration must be UTF-8").to_string()))
  }

  fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where E: de::Error {
    Ok(Value::Str(String::from_utf8(v).expect("Strings in the configuration must be UTF-8")))
  }

  fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'de> {
    let mut result = Vec::with_capacity(seq.size_hint().unwrap_or(0));
    while let Some(v) = seq.next_element()? {
      result.push(v);
    }
    Ok(Value::List(result))
  }

  fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
    let mut result = HashMap::with_capacity(map.size_hint().unwrap_or(0));
    while let Some((k, v)) = map.next_entry()? {
      result.insert(k, v);
    }
    Ok(Value::Map(result))
  }
}

impl<'de> Deserialize<'de> for Value {
  fn deserialize<D>(deserializer: D) -> Result<Value, D::Error> where D: Deserializer<'de> {
    deserializer.deserialize_any(ValueVisitor)
  }
}

fn parse_json(data: impl Read) {
  CONFIG.with(|config| {
    config.replace(Some(serde_json::from_reader(data).expect("Failed to parse configuration")));
  });
}

fn parse_yaml(data: impl Read) {
  CONFIG.with(|config| {
    config.replace(Some(serde_yaml::from_reader(data).expect("Failed to parse configuration")));
  });
}

//fn handle_serde(data: se)
#[cfg(test)]
mod tests {
  use super::parse_json;

  #[test]
  fn parse_json_works() {
    let json = r#"
    {
      "actions": {
        "Detect request errors with Nextcloud": [
          {
            "filter": "filter_equals",
            "args": { "field": "SYSLOG_IDENTIFIER", "value": "uwsgi" }
          },
          {
            "filter": "filter_pcre",
            "args": { "field": "MESSAGE", "re": "^\\[[^]]+\\] ([^ ]+) .*\\] ([A-Z]+ /[^?]*)(?:\\?.*)? => .*\\(HTTP/1.1 5..\\)", "save": [ "thatIP", "HTTPrequest" ] },
            "else": "… Report insufficient buffer-size for Nextcloud QUERY_STRING"
          },
          {
            "action": "action_dailyReport",
            "args": { "level": "INFO", "message": "IP {thatIP} failed to {HTTPrequest} on Nextcloud", "details": "FIRSTLAST" }
          }
        ]
      },
      "debug": false
    }
    "#.as_bytes();
    parse_json(json);
    super::CONFIG.with(|config| {
      println!("{:#?}", config.borrow());
    });
  }
}
