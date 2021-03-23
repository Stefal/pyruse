#[macro_use]
mod domain;
mod infra;
mod service;

use domain::action::{CounterRaise, CounterReset, DnatCapture, DnatReplace, Email, Log, Noop};
use domain::filter::Equals;
use domain::{ConfigPort, Counters, Modules, Workflow};
use infra::counter::InMemoryCounterAdapter;
use infra::dnat::InMemoryDnatMappingsAdapter;
use infra::log::SystemdLogAdapter;
use infra::{config::ConfFile, email::ProcessEmailAdapter};

type CountersImpl = InMemoryCounterAdapter;
type DnatImpl = InMemoryDnatMappingsAdapter;
type EmailImpl = ProcessEmailAdapter;
type LogImpl = SystemdLogAdapter;

fn main() {
  let mut conf = ConfFile::from_filesystem().to_config();
  let email = singleton_new!(EmailImpl::new(conf.get()));
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
  let gets_moved_into_closure = singleton_share!(email);
  modules.register_action(
    "action_email".to_string(),
    Box::new(move |a| {
      Box::new(Email::from_args(
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
