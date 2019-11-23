mod noop;
pub use self::noop::*;

/*
pub trait Action {
  fn act(&self, record: &mut Record) -> Result<(), ()>;
}

impl<T: Action> Module for T {
  fn run(&self, record: &mut Record) -> Result<bool, ()> {
    match self.act(record) {
      Ok(()) => Ok(true),
      Err(()) => Err(())
    }
  }
}
*/
