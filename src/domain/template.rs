use crate::domain::{Record, Value};
use regex::Regex;
use std::ops::{Index, Range};

pub struct Template {
  template: String,
  var_locations: Vec<Range<usize>>,
}

impl Template {
  pub fn new(template: String) -> Self {
    let var_locations = Regex::new(r"\{\w+\}")
      .unwrap()
      .find_iter(&template)
      .map(|m| m.range())
      .collect();
    Template {
      template,
      var_locations,
    }
  }

  pub fn format(&self, record: &Record) -> String {
    let tpl = &self.template;
    if self.var_locations.len() == 0 {
      return tpl.clone();
    }
    let mut message = String::with_capacity(tpl.len() * 2);
    let mut last_index = 0;
    for Range { start, end } in self.var_locations.iter() {
      message.push_str(tpl.index(last_index..*start));
      if let Some(Value::Str(s)) = record.get(tpl.index((start + 1)..(end - 1))) {
        message.push_str(s);
      } else {
        message.push_str(tpl.index(*start..*end));
      }
      last_index = *end;
    }
    if last_index < tpl.len() {
      message.push_str(tpl.index(last_index..tpl.len()));
    }
    message
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::{Template, Value};
  use core::panic;
  use std::collections::HashMap;

  #[test]
  fn template_without_placeholder_is_rendered_as_is() {
    let template = Template::new("x".into());
    let result = template.format(&HashMap::new());
    assert_eq!("x", &result);
  }

  #[test]
  fn placeholder_without_variable_is_rendered_as_is() {
    let template = Template::new("x{y}z".into());
    let result = template.format(&HashMap::new());
    assert_eq!("x{y}z", &result);
  }

  #[test]
  fn placeholder_with_variable_is_replaced() {
    let template = Template::new("x{y}z".into());
    let mut record = HashMap::new();
    record.insert("y".into(), Value::Str("y".into()));
    let result = template.format(&record);
    assert_eq!("xyz", &result);
  }

  #[test]
  fn all_variables_are_replaced() {
    let template = Template::new("{x}{a}{yy}-{zzz}".into());
    let mut record = HashMap::new();
    record.insert("x".into(), Value::Str("x".into()));
    record.insert("yy".into(), Value::Str("y".into()));
    record.insert("zzz".into(), Value::Str("z".into()));
    let result = template.format(&record);
    assert_eq!("x{a}y-z", &result);
  }
}
