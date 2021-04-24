#[macro_use]
mod domain;
mod infra;
mod service;

use domain::action::{
  CounterRaise, CounterReset, DnatCapture, DnatReplace, Email, Log, NetfilterBan, Noop,
};
use domain::filter::Equals;
use domain::{ConfigPort, Counters, Modules, Workflow};
use infra::config::{configuration_from_filesystem, SerdeConfigAdapter, ETC_PATH};
use infra::counter::InMemoryCounterAdapter;
use infra::dnat::InMemoryDnatMappingsAdapter;
use infra::email::ProcessEmailAdapter;
use infra::log::SystemdLogAdapter;
use infra::netfilter::{
  FilesystemNetfilterStorageAdapter, IpsetNetfilterBackendAdapter, NftablesNetfilterBackendAdapter,
};

type CountersImpl = InMemoryCounterAdapter;
type DnatImpl = InMemoryDnatMappingsAdapter;
type EmailImpl = ProcessEmailAdapter;
type IpsetBackendImpl = IpsetNetfilterBackendAdapter;
type IpsetStorageImpl = FilesystemNetfilterStorageAdapter;
type LogImpl = SystemdLogAdapter;
type NftablesBackendImpl = NftablesNetfilterBackendAdapter;
type NftablesStorageImpl = FilesystemNetfilterStorageAdapter;

fn main() {
  let mut conf: SerdeConfigAdapter = configuration_from_filesystem(ETC_PATH).into();
  let email = singleton_new!(EmailImpl::new(conf.get()));
  let log = singleton_new!(LogImpl::open().expect("Error initializing systemd"));
  let mut modules = Modules::new();
  let counters = singleton_new!(Counters::<CountersImpl>::new(CountersImpl::new()));
  let dnat = singleton_new!(DnatImpl::new());
  let gets_moved_into_closure = singleton_share!(counters);
  let ipset_backend = singleton_new!(IpsetBackendImpl::new(conf.get()));
  let ipset_storage = singleton_new!(IpsetStorageImpl::new(conf.get(), "action_ipsetBan.json"));
  let nftables_backend = singleton_new!(NftablesBackendImpl::new(conf.get()));
  let nftables_storage = singleton_new!(NftablesStorageImpl::new(conf.get(), "action_nftBan.json"));
  modules.register_action(
    "action_counterRaise".into(),
    Box::new(move |a| {
      Box::new(CounterRaise::<CountersImpl>::from_args(
        a,
        singleton_share!(gets_moved_into_closure), // clone for each call of the constructor
      ))
    }),
  );
  let gets_moved_into_closure = singleton_share!(counters);
  modules.register_action(
    "action_counterReset".into(),
    Box::new(move |a| {
      Box::new(CounterReset::<CountersImpl>::from_args(
        a,
        singleton_share!(gets_moved_into_closure), // clone for each call of the constructor
      ))
    }),
  );
  let gets_moved_into_closure = singleton_share!(dnat);
  modules.register_action(
    "action_dnatCapture".into(),
    Box::new(move |a| {
      Box::new(DnatCapture::from_args(
        a,
        singleton_share!(gets_moved_into_closure), // clone for each call of the constructor
      ))
    }),
  );
  let gets_moved_into_closure = singleton_share!(dnat);
  modules.register_action(
    "action_dnatReplace".into(),
    Box::new(move |a| {
      Box::new(DnatReplace::from_args(
        a,
        singleton_share!(gets_moved_into_closure), // clone for each call of the constructor
      ))
    }),
  );
  let gets_moved_into_closure = singleton_share!(email);
  modules.register_action(
    "action_email".into(),
    Box::new(move |a| {
      Box::new(Email::from_args(
        a,
        singleton_share!(gets_moved_into_closure), // clone for each call of the constructor
      ))
    }),
  );
  let gets_moved_into_closure = singleton_share!(ipset_backend);
  let gets_moved_into_closure_2 = singleton_share!(ipset_storage);
  modules.register_action(
    "action_ipsetBan".into(),
    Box::new(move |a| {
      Box::new(NetfilterBan::from_args(
        a,
        "action_ipsetBan",
        "ipSetIPv4",
        "ipSetIPv6",
        singleton_share!(gets_moved_into_closure), // clone for each call of the constructor
        singleton_share!(gets_moved_into_closure_2), // clone for each call of the constructor
      ))
    }),
  );
  modules.register_action(
    "action_log".into(),
    Box::new(move |a| Box::new(Log::from_args(a, singleton_share!(log)))),
  );
  let gets_moved_into_closure = singleton_share!(nftables_backend);
  let gets_moved_into_closure_2 = singleton_share!(nftables_storage);
  modules.register_action(
    "action_nftBan".into(),
    Box::new(move |a| {
      Box::new(NetfilterBan::from_args(
        a,
        "action_nftBan",
        "nftSetIPv4",
        "nftSetIPv6",
        singleton_share!(gets_moved_into_closure), // clone for each call of the constructor
        singleton_share!(gets_moved_into_closure_2), // clone for each call of the constructor
      ))
    }),
  );
  modules.register_action(
    "action_noop".into(),
    Box::new(move |a| Box::new(Noop::from_args(a))),
  );
  modules.register_filter(
    "filter_equals".into(),
    Box::new(move |a| Box::new(Equals::from_args(a))),
  );
  let _workflow = Workflow::build(conf.get(), &modules);
  println!("Hello, world!");
}
