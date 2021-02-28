mod domain;
mod infra;
mod service;

use crate::domain::action::Log;
use crate::domain::action::Noop;
use crate::domain::filter::Equals;
use crate::domain::{ConfigPort, Modules, Workflow};
use crate::infra::config::ConfFile;
use crate::infra::log::SystemdLogAdapter;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
  let mut conf = ConfFile::from_filesystem().to_config();
  let log = Rc::new(RefCell::new(
    SystemdLogAdapter::open().expect("Error initializing systemd"),
  ));
  let mut modules = Modules::new();
  modules.register_action(
    "action_noop".to_string(),
    Box::new(move |a| Box::new(Noop::from_args(a))),
  );
  modules.register_action(
    "action_log".to_string(),
    Box::new(move |a| Box::new(Log::from_args(a, log.clone()))),
  );
  modules.register_filter(
    "filter_equals".to_string(),
    Box::new(move |a| Box::new(Equals::from_args(a))),
  );
  let _workflow = Workflow::build(conf.get(), &modules);
  println!("Hello, world!");
}
