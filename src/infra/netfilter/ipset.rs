use crate::domain::{action::NetfilterBackendPort, Config, Error};
use chrono::{DateTime, Utc};
use std::ffi::OsString;
use std::process::Command;

const DEFAULT_IPSET: &[&str] = &["/usr/bin/ipset", "-exist", "-quiet"];

pub struct IpsetNetfilterBackendAdapter {
  ipset_cmd: OsString,
  ipset_params: Vec<OsString>,
}
impl IpsetNetfilterBackendAdapter {
  pub fn new(conf: &Config) -> IpsetNetfilterBackendAdapter {
    let (ipset_cmd, ipset_params) =
      super::read_command_from_options(conf, "ipsetBan", "ipset", DEFAULT_IPSET).unwrap();
    IpsetNetfilterBackendAdapter {
      ipset_cmd,
      ipset_params,
    }
  }
}

impl NetfilterBackendPort for IpsetNetfilterBackendAdapter {
  fn set_ban<'s, 'r>(
    &'s mut self,
    nf_set: &'s str,
    ip: &'r str,
    ban_until: &'s Option<DateTime<Utc>>,
  ) -> Result<(), Error> {
    let mut ipset = Command::new(&self.ipset_cmd);
    for p in &self.ipset_params {
      ipset.arg(p);
    }
    ipset.arg("add");
    ipset.arg(nf_set);
    ipset.arg(ip);
    let debug_seconds = if let Some(d) = ban_until {
      let seconds = (d.timestamp() - Utc::now().timestamp()).to_string();
      ipset.arg("timeout");
      ipset.arg(&seconds);
      seconds
    } else {
      "indefinitely".into()
    };
    let exit_status = ipset.spawn()?.wait()?;
    if exit_status.success() {
      Ok(())
    } else {
      Err(Error::from(format!(
        "Ipset ban {:?} failed with code {:?}",
        &["add", nf_set, ip, &debug_seconds],
        exit_status.code()
      )))
    }
  }

  fn cancel_ban<'s, 'r>(&'s mut self, nf_set: &'s str, ip: &'r str) -> Result<(), Error> {
    let mut ipset = Command::new(&self.ipset_cmd);
    for p in &self.ipset_params {
      ipset.arg(p);
    }
    ipset.arg("del");
    ipset.arg(nf_set);
    ipset.arg(ip);
    let exit_status = ipset.spawn()?.wait()?;
    if exit_status.success() {
      Ok(())
    } else {
      Err(Error::from(format!(
        "Ipset unban {:?} failed with code {:?}",
        &["del", nf_set, ip],
        exit_status.code()
      )))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::IpsetNetfilterBackendAdapter;
  use crate::domain::action::NetfilterBackendPort;
  use crate::domain::{Config, Value};
  use chrono::{Duration, Utc};
  use std::collections::HashMap;
  use std::{env, fs};

  #[test]
  fn ban_with_enddate_has_a_timeout() {
    let mut temp = env::temp_dir();
    temp.push("ipset-ban-test-with-time.cmd");
    let filename = temp.to_str().unwrap();
    let conf = test_config(filename);
    let mut adapter = IpsetNetfilterBackendAdapter::new(&conf);
    adapter
      .set_ban(
        "an iptables set",
        "::1",
        &Some(Utc::now() + Duration::seconds(9)),
      )
      .unwrap();
    let file = fs::read_to_string(filename).unwrap();
    let prefix = "add\nan iptables set\n::1\ntimeout\n";
    assert!(file.starts_with(prefix));
    assert!(file[prefix.len()..(file.len() - 1)].parse::<i32>().unwrap() < 10); // ignore trailing "\n"
    fs::remove_file(filename).unwrap();
  }

  #[test]
  fn ban_without_enddate_has_no_timeout() {
    let mut temp = env::temp_dir();
    temp.push("ipset-ban-test-without-time.cmd");
    let filename = temp.to_str().unwrap();
    let conf = test_config(filename);
    let mut adapter = IpsetNetfilterBackendAdapter::new(&conf);
    adapter.set_ban("an iptables set", "::1", &None).unwrap();
    let file = fs::read_to_string(filename).unwrap();
    assert_eq!(&file, "add\nan iptables set\n::1\n");
    fs::remove_file(filename).unwrap();
  }

  #[test]
  fn unban_works() {
    let mut temp = env::temp_dir();
    temp.push("ipset-unban-test.cmd");
    let filename = temp.to_str().unwrap();
    let conf = test_config(filename);
    let mut adapter = IpsetNetfilterBackendAdapter::new(&conf);
    adapter.cancel_ban("an iptables set", "1.2.3.4").unwrap();
    let file = fs::read_to_string(filename).unwrap();
    assert_eq!(&file, "del\nan iptables set\n1.2.3.4\n");
    fs::remove_file(filename).unwrap();
  }

  fn test_config(filename: &str) -> Config {
    let ipset = &[
      "bash",
      "-c",
      &format!(r#"printf '%s\n' "$@" >"{}""#, filename),
      "-",
    ];
    let mut options = HashMap::new();
    let mut ipset_ban = HashMap::new();
    ipset_ban.insert(
      "ipset".into(),
      Value::List(ipset.iter().map(|s| Value::Str((*s).into())).collect()),
    );
    options.insert("ipsetBan".into(), Value::Map(ipset_ban));
    Config::new(None, Some(options))
  }
}
