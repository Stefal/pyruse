use crate::domain::{action::NetfilterBackendPort, Config, Error};
use chrono::{DateTime, Utc};
use std::ffi::OsString;
use std::process::Command;

const DEFAULT_NFT: &[&str] = &["/usr/bin/nft"];

pub struct NftablesNetfilterBackendAdapter {
  nft_cmd: OsString,
  nft_params: Vec<OsString>,
}
impl NftablesNetfilterBackendAdapter {
  pub fn new(conf: &Config) -> NftablesNetfilterBackendAdapter {
    let (nft_cmd, nft_params) =
      super::read_command_from_options(conf, "nftBan", "nft", DEFAULT_NFT).unwrap();
    NftablesNetfilterBackendAdapter {
      nft_cmd,
      nft_params,
    }
  }
}

impl NetfilterBackendPort for NftablesNetfilterBackendAdapter {
  fn set_ban<'s, 'r>(
    &'s mut self,
    nf_set: &'s str,
    ip: &'r str,
    ban_until: &'s Option<DateTime<Utc>>,
  ) -> Result<(), Error> {
    let mut nft = Command::new(&self.nft_cmd);
    for p in &self.nft_params {
      nft.arg(p);
    }
    let timeout = ban_until
      .map(|d| format!(" timeout {}s", d.timestamp() - Utc::now().timestamp()))
      .unwrap_or(String::new());
    let ban = format!("add element {} {{{}{}}}", nf_set, ip, timeout);
    nft.arg(&ban);
    let exit_status = nft.spawn()?.wait()?;
    if exit_status.success() {
      Ok(())
    } else {
      Err(Error::from(format!(
        "Nftables ban [{}] failed with code {:?}",
        ban,
        exit_status.code()
      )))
    }
  }

  fn cancel_ban<'s, 'r>(&'s mut self, nf_set: &'s str, ip: &'r str) -> Result<(), Error> {
    let mut nft = Command::new(&self.nft_cmd);
    for p in &self.nft_params {
      nft.arg(p);
    }
    let unban = format!("delete element {} {{{}}}", nf_set, ip);
    nft.arg(&unban);
    let exit_status = nft.spawn()?.wait()?;
    if exit_status.success() {
      Ok(())
    } else {
      Err(Error::from(format!(
        "Nftables unban [{}] failed with code {:?}",
        unban,
        exit_status.code()
      )))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::NftablesNetfilterBackendAdapter;
  use crate::domain::action::NetfilterBackendPort;
  use crate::domain::{Config, Value};
  use chrono::{Duration, Utc};
  use std::collections::HashMap;
  use std::{env, fs};

  #[test]
  fn ban_with_enddate_has_a_timeout() {
    let mut temp = env::temp_dir();
    temp.push("nft-ban-test-with-time.cmd");
    let filename = temp.to_str().unwrap();
    let conf = test_config(filename);
    let mut adapter = NftablesNetfilterBackendAdapter::new(&conf);
    adapter
      .set_ban("a nft set", "::1", &Some(Utc::now() + Duration::seconds(9)))
      .unwrap();
    let file = fs::read_to_string(filename).unwrap();
    let prefix = "add element a nft set {::1 timeout ";
    assert!(file.starts_with(prefix));
    assert!(file[prefix.len()..(file.len() - 3)].parse::<i32>().unwrap() < 10); // ignore trailing "s}\n"
    fs::remove_file(filename).unwrap();
  }

  #[test]
  fn ban_without_enddate_has_no_timeout() {
    let mut temp = env::temp_dir();
    temp.push("nft-ban-test-without-time.cmd");
    let filename = temp.to_str().unwrap();
    let conf = test_config(filename);
    let mut adapter = NftablesNetfilterBackendAdapter::new(&conf);
    adapter.set_ban("a nft set", "::1", &None).unwrap();
    let file = fs::read_to_string(filename).unwrap();
    assert_eq!(&file, "add element a nft set {::1}\n");
    fs::remove_file(filename).unwrap();
  }

  #[test]
  fn unban_works() {
    let mut temp = env::temp_dir();
    temp.push("nft-unban-test.cmd");
    let filename = temp.to_str().unwrap();
    let conf = test_config(filename);
    let mut adapter = NftablesNetfilterBackendAdapter::new(&conf);
    adapter.cancel_ban("a nft set", "1.2.3.4").unwrap();
    let file = fs::read_to_string(filename).unwrap();
    assert_eq!(&file, "delete element a nft set {1.2.3.4}\n");
    fs::remove_file(filename).unwrap();
  }

  fn test_config(filename: &str) -> Config {
    let nft = &[
      "bash",
      "-c",
      &format!(r#"printf '%s\n' "$@" >"{}""#, filename),
      "-",
    ];
    let mut options = HashMap::new();
    let mut nft_ban = HashMap::new();
    nft_ban.insert(
      "nft".into(),
      Value::List(nft.iter().map(|s| Value::Str((*s).into())).collect()),
    );
    options.insert("nftBan".into(), Value::Map(nft_ban));
    Config::new(None, Some(options))
  }
}
