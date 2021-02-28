use crate::domain::{Action, ModuleArgs, Record};

pub struct Noop {}

impl Noop {
  pub fn from_args(_args: ModuleArgs) -> Noop {
    Noop {}
  }
}

impl Action for Noop {
  fn act(&mut self, _record: &mut Record) -> Result<(), ()> {
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::action::Noop;
  use crate::domain::{Action, ModuleArgs, Record};
  use std::collections::HashMap;

  fn generate_empty_args_record() -> (ModuleArgs, Record) {
    let args = HashMap::with_capacity(0);
    let record = HashMap::with_capacity(0);
    (args, record)
  }

  #[test]
  fn noop_does_nothing() {
    // Given
    let (args, mut record) = generate_empty_args_record();
    let mut action = Noop::from_args(args);

    // Then
    assert_eq!((), action.act(&mut record).unwrap());
  }
}
