use crate::modules::{Module,ModuleArgs};
use crate::common::Record;

#[derive(Debug)]
pub struct Noop {}

impl Noop {
  pub fn from_args(mut _args: ModuleArgs) -> Noop {
    Noop {}
  }
}

impl Module for Noop {
  fn run(&self, _record: &mut Record) -> Result<bool, ()> {
    Ok(true)
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use crate::common::Record;
  use crate::actions::Noop;
  use crate::modules::{Module,ModuleArgs};

  fn generate_empty_args_record() -> (ModuleArgs<'static>, Record<'static>) {
    let args = HashMap::with_capacity(0);
    let record = HashMap::with_capacity(0);
    (args, record)
  }

  #[test]
  fn noop_does_nothing() {
    let (args, mut record) = generate_empty_args_record();
    let action = Noop::from_args(args);
    assert!(action.run(&mut record).unwrap());
  }
}
