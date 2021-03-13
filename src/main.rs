#[macro_use]
mod domain;
mod infra;
mod service;

use crate::domain::action::{CounterRaise, CounterReset, Log, Noop};
use crate::domain::filter::Equals;
use crate::domain::{ConfigPort, Counters, Modules, Workflow};
use crate::infra::config::ConfFile;
use crate::infra::counter::InMemoryCounterAdapter;
use crate::infra::log::SystemdLogAdapter;

type CountersImpl = InMemoryCounterAdapter;
type LogImpl = SystemdLogAdapter;

fn main() {
  let mut conf = ConfFile::from_filesystem().to_config();
  let log = singleton_new!(LogImpl::open().expect("Error initializing systemd"));
  let mut modules = Modules::new();
  let counters = singleton_new!(Counters::<CountersImpl>::new(CountersImpl::new()));
  let gets_moved_into_closure = singleton_share!(counters);
  modules.register_action(
    "action_counterRaise".to_string(),
    Box::new(move |a| {
      Box::new(CounterRaise::<CountersImpl>::from_args(
        a,
        singleton_share!(gets_moved_into_closure), // clone for each call of the constructor
      ))
    }),
  );
  let gets_moved_into_closure = singleton_share!(counters);
  modules.register_action(
    "action_counterReset".to_string(),
    Box::new(move |a| {
      Box::new(CounterReset::<CountersImpl>::from_args(
        a,
        singleton_share!(gets_moved_into_closure), // clone for each call of the constructor
      ))
    }),
  );
  modules.register_action(
    "action_log".to_string(),
    Box::new(move |a| Box::new(Log::from_args(a, singleton_share!(log)))),
  );
  modules.register_action(
    "action_noop".to_string(),
    Box::new(move |a| Box::new(Noop::from_args(a))),
  );
  modules.register_filter(
    "filter_equals".to_string(),
    Box::new(move |a| Box::new(Equals::from_args(a))),
  );
  let _workflow = Workflow::build(conf.get(), &modules);
  println!("Hello, world!");
}
