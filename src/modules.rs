use crate::common::Record;
use crate::{actions,filters};

struct Available {
  name: &'static str,
  cons: fn(ModuleArgs) -> Box<dyn Module>
}

const AVAILABLE: &[Available] = &[
  Available { name: "action_noop", cons: move |a| Box::new(actions::Noop::from_args(a)) },
  Available { name: "filter_equals", cons: move |a| Box::new(filters::Equals::from_args(a)) }
];

pub trait Module {
  fn run(&self, record: &mut Record) -> Result<bool, ()>;
}

pub type ModuleArgs<'a> = Record<'a>;

pub enum ModuleType {
  Filter,
  Action
}
