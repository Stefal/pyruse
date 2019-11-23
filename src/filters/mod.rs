mod equals;
pub use self::equals::*;

/*
pub trait Filter {
  fn filter(&self, record: &mut Record) -> bool;
}

impl<T: Filter> Module for T {
  fn run(&self, record: &mut Record) -> Result<bool, ()> {
    Ok(self.filter(record))
  }
}
*/
