use crate::domain::Error;
use serde::{de::DeserializeOwned, Serialize};
use std::io::{Read, Write};

pub enum Input<R: Read> {
  Json(R),
  Yaml(R),
}
impl<R: Read> Input<R> {
  pub fn parse<T: DeserializeOwned>(self) -> Result<T, Error> {
    match self {
      Input::Json(r) => Ok(serde_json::from_reader(r)?),
      Input::Yaml(r) => Ok(serde_yaml::from_reader(r)?),
    }
  }
}

pub enum Output<W: Write> {
  Json(W),
  Yaml(W),
}
impl<W: Write> Output<W> {
  pub fn format<T: Serialize>(self, data: T) -> Result<(), Error> {
    match self {
      Output::Json(w) => Ok(serde_json::to_writer(w, &data)?),
      Output::Yaml(w) => Ok(serde_yaml::to_writer(w, &data)?),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{Input, Output};
  use crate::domain::test_util::WriteProxy;

  #[test]
  fn parse_on_json_input_gives_data_if_syntax_is_correct() {
    let raw = "[1,3,5]";
    let input = Input::Json(raw.as_bytes());
    let data: Vec<u64> = input.parse().unwrap();
    assert_eq!(vec![1, 3, 5], data);
  }

  #[test]
  fn parse_on_yaml_input_gives_data_if_syntax_is_correct() {
    let raw = "- 1\n- 3\n- 5\n";
    let input = Input::Yaml(raw.as_bytes());
    let data: Vec<u64> = input.parse().unwrap();
    assert_eq!(vec![1, 3, 5], data);
  }

  #[test]
  fn format_on_json_gives_json_output() {
    let data = vec![1, 3, 5];
    let mut bytes = Vec::<u8>::new();
    let writer = WriteProxy::new(&mut bytes);
    Output::Json(writer).format(data).unwrap();
    assert_eq!("[1,3,5]", String::from_utf8(bytes).unwrap());
  }

  #[test]
  fn format_on_yaml_gives_yaml_output() {
    let data = vec![1, 3, 5];
    let mut bytes = Vec::<u8>::new();
    let writer = WriteProxy::new(&mut bytes);
    Output::Yaml(writer).format(data).unwrap();
    assert_eq!("---\n- 1\n- 3\n- 5\n", String::from_utf8(bytes).unwrap());
  }
}
