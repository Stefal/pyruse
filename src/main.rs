#[macro_use]
mod domain;
mod infra;
mod service;

use domain::action::{CounterRaise, CounterReset, DnatCapture, DnatReplace, Log, Noop};
use domain::filter::Equals;
use domain::{ConfigPort, Counters, Modules, Workflow};
use infra::config::ConfFile;
use infra::counter::InMemoryCounterAdapter;
use infra::dnat::InMemoryDnatMappingsAdapter;
use infra::log::SystemdLogAdapter;

type CountersImpl = InMemoryCounterAdapter;
type DnatImpl = InMemoryDnatMappingsAdapter;
type LogImpl = SystemdLogAdapter;

fn main() {
  let mut conf = ConfFile::from_filesystem().to_config();
  let log = singleton_new!(LogImpl::open().expect("Error initializing systemd"));
  let mut modules = Modules::new();
  let counters = singleton_new!(Counters::<CountersImpl>::new(CountersImpl::new()));
  let dnat = singleton_new!(DnatImpl::new());
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
  let gets_moved_into_closure = singleton_share!(dnat);
  modules.register_action(
    "action_dnatCapture".to_string(),
    Box::new(move |a| {
      Box::new(DnatCapture::from_args(
        a,
        singleton_share!(gets_moved_into_closure), // clone for each call of the constructor
      ))
    }),
  );
  let gets_moved_into_closure = singleton_share!(dnat);
  modules.register_action(
    "action_dnatReplace".to_string(),
    Box::new(move |a| {
      Box::new(DnatReplace::from_args(
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
