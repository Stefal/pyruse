use crate::domain::{Action, Error, ModuleArgs, Record, Singleton, Value};
use chrono::{DateTime, Duration, Utc};

pub trait NetfilterBackendPort {
  fn set_ban<'s, 'r>(
    &'s mut self,
    nf_set: &'s str,
    ip: &'r str,
    ban_until: &'s Option<DateTime<Utc>>,
  ) -> Result<(), Error>;
  fn cancel_ban<'s, 'r>(&'s mut self, nf_set: &'s str, ip: &'r str) -> Result<(), Error>;
}

pub trait NetfilterStoragePort {
  fn store_and_remove_obsoletes<'s, 'r>(
    &'s mut self,
    nf_set: &'s str,
    ip: &'r str,
    ban_until: &'s Option<DateTime<Utc>>,
  ) -> Result<bool, Error>;
}

pub struct NetfilterBan {
  backend: Singleton<dyn NetfilterBackendPort>,
  storage: Singleton<dyn NetfilterStoragePort>,
  ipv4_set: String,
  ipv6_set: String,
  field: String,
  ban_seconds: Option<usize>,
}
impl NetfilterBan {
  pub fn from_args(
    mut args: ModuleArgs,
    module_alias: &str,
    ipv4_arg_name: &str,
    ipv6_arg_name: &str,
    backend: Singleton<dyn NetfilterBackendPort>,
    storage: Singleton<dyn NetfilterStoragePort>,
  ) -> NetfilterBan {
    let ipv4_set = match args.remove(ipv4_arg_name) {
      Some(Value::Str(s)) => s,
      _ => panic!(
        "The {} action needs an IPv4 set name in “{}”",
        module_alias, ipv4_arg_name
      ),
    };
    let ipv6_set = match args.remove(ipv6_arg_name) {
      Some(Value::Str(s)) => s,
      _ => panic!(
        "The {} action needs an IPv6 set name in “{}”",
        module_alias, ipv4_arg_name
      ),
    };
    let field = match args.remove("IP") {
      Some(Value::Str(s)) => s,
      _ => panic!(
        "The {} action needs a field to read the IP address from, in “IP”",
        module_alias
      ),
    };
    let ban_seconds = match args.remove("banSeconds") {
      Some(Value::Int(i)) => Some(i as usize),
      _ => None,
    };
    NetfilterBan {
      backend,
      storage,
      ipv4_set,
      ipv6_set,
      field,
      ban_seconds,
    }
  }
}

impl Action for NetfilterBan {
  fn act(&mut self, record: &mut Record) -> Result<(), Error> {
    if let Some(Value::Str(ip)) = record.get(&self.field) {
      let set = if ip.contains(':') {
        &self.ipv6_set
      } else {
        &self.ipv4_set
      };
      let ban_until = self
        .ban_seconds
        .map(|s| Utc::now() + Duration::seconds(s as i64));
      if self
        .storage
        .borrow_mut()
        .store_and_remove_obsoletes(set, ip, &ban_until)?
      {
        // should not happen, since the IP is banned…
        self
          .backend
          .borrow_mut()
          .cancel_ban(set, ip)
          .unwrap_or_default(); // if too late: not a problem
      }
      self.backend.borrow_mut().set_ban(set, ip, &ban_until)
    } else {
      Ok(())
    }
  }
}
