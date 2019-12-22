use crate::modules::{Action,AvailableAction,ModuleArgs};
use crate::common::Record;

#[derive(Debug)]
pub struct Noop {}

inventory::submit! {
  AvailableAction::new("action_noop", move |a| Box::new(Noop::from_args(a)))
}

impl Noop {
  pub fn from_args(mut _args: ModuleArgs) -> Noop {
    Noop {}
  }
}

impl Action for Noop {
  fn act(&self, _record: &mut Record) -> Result<(), ()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use crate::common::Record;
  use crate::actions::Noop;
  use crate::modules::{Action,ModuleArgs};

  fn generate_empty_args_record() -> (ModuleArgs, Record<'static>) {
    let args = HashMap::with_capacity(0);
    let record = HashMap::with_capacity(0);
    (args, record)
  }

  #[test]
  fn noop_does_nothing() {
    let (args, mut record) = generate_empty_args_record();
    let action = Noop::from_args(args);
    assert_eq!((), action.act(&mut record).unwrap());
  }
}
