use crate::domain::action::NetfilterStoragePort;
use crate::domain::{Config, Error, Value};
use crate::infra::file::DataFile;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, path::PathBuf};

pub struct FilesystemNetfilterStorageAdapter {
  file: DataFile,
}
impl FilesystemNetfilterStorageAdapter {
  pub fn new(conf: &Config, basename: &str) -> FilesystemNetfilterStorageAdapter {
    let dirname = PathBuf::from(if let Some(Value::Str(s)) = conf.options.get("storage") {
      s.as_ref()
    } else {
      "/var/lib/pyruse"
    });
    FilesystemNetfilterStorageAdapter {
      file: DataFile::try_from((&dirname, basename)).unwrap(),
    }
  }
}

impl NetfilterStoragePort for FilesystemNetfilterStorageAdapter {
  fn store_and_remove_obsoletes<'s, 'r>(
    &'s mut self,
    nf_set: &'s str,
    ip: &'r str,
    ban_until: &'s Option<DateTime<Utc>>,
  ) -> Result<bool, Error> {
    let now = Utc::now();
    let mut found = false;
    let mut bans = (if self.file.exists() {
      self.file.open_r()?.parse::<Vec<Ban>>()?
    } else {
      Vec::new()
    })
    .drain(..)
    .filter(|b| b.ban_until.map(|d| d > now).unwrap_or(true))
    .filter(|b| {
      if (&b.ip.as_ref(), &b.nf_set.as_ref()) == (&ip, &nf_set) {
        found = true;
        false
      } else {
        true
      }
    })
    .collect::<Vec<Ban>>();
    bans.push(Ban {
      ip: ip.to_string(),
      nf_set: nf_set.to_string(),
      ban_until: ban_until.clone(),
    });
    self.file.open_w()?.format(bans)?;
    Ok(found)
  }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Ban {
  #[serde(rename = "IP")]
  ip: String,
  #[serde(rename = "nfSet")]
  nf_set: String,
  #[serde(rename = "timestamp", with = "timestamp_serde")]
  ban_until: Option<DateTime<Utc>>,
}

// Lossy de·serializer which does not care about sub-seconds
mod timestamp_serde {
  use chrono::{DateTime, NaiveDateTime, Utc};
  use serde::de::{self, Visitor};
  use serde::{Deserializer, Serializer};
  use std::fmt;

  struct TimestampVisitor;

  impl<'de> Visitor<'de> for TimestampVisitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("a number")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v as i64)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v as i64)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v as i64)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v)
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v as i64)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v as i64)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v as i64)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v as i64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v as i64)
    }

    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v as i64)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v as i64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      Ok(v as i64)
    }
  }

  pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_i64(date.map(|d| d.timestamp()).unwrap_or(0))
  }

  pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
  where
    D: Deserializer<'de>,
  {
    match deserializer.deserialize_any(TimestampVisitor) {
      Ok(i) if i == 0 => Ok(None),
      Ok(i) => Ok(Some(DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(i, 0),
        Utc,
      ))),
      Err(x) => Err(x),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{timestamp_serde, Ban, FilesystemNetfilterStorageAdapter};
  use crate::domain::action::NetfilterStoragePort;
  use crate::domain::test_util::WriteProxy;
  use crate::domain::{Config, Value};
  use crate::infra::file::DataFile;
  use crate::infra::serde::{Input, Output};
  use chrono::{DateTime, Utc};
  use serde::{Deserialize, Serialize};
  use std::convert::TryFrom;
  use std::env;
  use std::fs;
  use std::path::PathBuf;
  use std::time::{Duration, UNIX_EPOCH};

  #[derive(Serialize, Deserialize)]
  struct TimestampHolder {
    #[serde(with = "timestamp_serde")]
    ts: Option<DateTime<Utc>>,
  }

  #[test]
  fn timestamp_subsecond_is_discarded_when_it_is_read() {
    let yaml = "ts: 1617864646.521".as_bytes();
    let value: TimestampHolder = Input::Yaml(yaml).parse().unwrap();
    assert_eq!(1617864646, value.ts.unwrap().timestamp());
    assert_eq!(0, value.ts.unwrap().timestamp_subsec_micros());
  }

  #[test]
  fn timestamp_subsecond_is_not_written() {
    let value: TimestampHolder = TimestampHolder {
      ts: Some(DateTime::from(
        UNIX_EPOCH + Duration::from_millis(1618655010521),
      )),
    };
    let mut bytes = Vec::<u8>::new();
    let writer = WriteProxy::new(&mut bytes);
    Output::Yaml(writer).format(value).unwrap();
    assert_eq!("---\nts: 1618655010\n", String::from_utf8(bytes).unwrap());
  }

  #[test]
  fn zero_timestamp_is_read_as_an_empty_datetime() {
    let yaml = "ts: 0".as_bytes();
    let value: TimestampHolder = Input::Yaml(yaml).parse().unwrap();
    assert_eq!(None, value.ts);
  }

  #[test]
  fn empty_timestamp_is_written_as_zero() {
    let value: TimestampHolder = TimestampHolder { ts: None };
    let mut bytes = Vec::<u8>::new();
    let writer = WriteProxy::new(&mut bytes);
    Output::Yaml(writer).format(value).unwrap();
    assert_eq!("---\nts: 0\n", String::from_utf8(bytes).unwrap());
  }

  #[test]
  fn ban_is_correctly_read() {
    let json =
      r#"{"IP": "164.52.24.168", "nfSet": "ip Inet4 mail_ban", "timestamp": 1614263304.356016}"#
        .as_bytes();
    let value: Ban = Input::Json(json).parse().unwrap();
    let expected = Ban {
      ip: "164.52.24.168".into(),
      nf_set: "ip Inet4 mail_ban".into(),
      ban_until: Some(DateTime::from(UNIX_EPOCH + Duration::from_secs(1614263304))),
    };
    assert_eq!(expected, value);
  }

  #[test]
  fn ban_is_correctly_written() {
    let ban = Ban {
      ip: "164.52.24.168".into(),
      nf_set: "ip Inet4 mail_ban".into(),
      ban_until: Some(DateTime::from(UNIX_EPOCH + Duration::from_secs(1614263304))),
    };
    let mut bytes = Vec::<u8>::new();
    let writer = WriteProxy::new(&mut bytes);
    Output::Json(writer).format(ban).unwrap();
    assert_eq!(
      r#"{"IP":"164.52.24.168","nfSet":"ip Inet4 mail_ban","timestamp":1614263304}"#,
      String::from_utf8(bytes).unwrap()
    );
  }

  #[test]
  fn ban_file_is_correctly_read() {
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    path.push("tests/netfilter-data.yaml");
    let file = fs::File::open(path).unwrap();
    let bans: Vec<Ban> = Input::Yaml(file).parse().unwrap();
    assert_eq!(5, bans.len());
  }

  #[test]
  fn ban_file_is_correctly_filtered_and_new_ban_added_to_file() {
    let mut srcpath = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    srcpath.push("tests/netfilter-data.yaml");
    let mut destpath = env::temp_dir();
    destpath.push("netfilter-data-copy-for-new.yaml");
    fs::copy(&srcpath, &destpath).unwrap();
    let mut storage = FilesystemNetfilterStorageAdapter {
      file: DataFile::Yaml(destpath.clone().into()),
    };
    let found = storage
      .store_and_remove_obsoletes("NEW SET", "NEW.IP", &None)
      .unwrap();
    let file = fs::File::open(&destpath).unwrap();
    let bans: Vec<Ban> = Input::Yaml(file).parse().unwrap();
    let expected = vec![
      Ban {
        ip: "121.66.35.37".into(),
        nf_set: "ip Inet4 mail_ban".into(),
        ban_until: Some(DateTime::from(UNIX_EPOCH + Duration::from_secs(2614199470))),
      },
      Ban {
        ip: "59.39.183.34".into(),
        nf_set: "ip Inet4 mail_ban".into(),
        ban_until: Some(DateTime::from(UNIX_EPOCH + Duration::from_secs(2614201649))),
      },
      Ban {
        ip: "51.11.240.49".into(),
        nf_set: "ip Inet4 https_ban".into(),
        ban_until: None,
      },
      Ban {
        ip: "NEW.IP".into(),
        nf_set: "NEW SET".into(),
        ban_until: None,
      },
    ];
    fs::remove_file(&destpath).unwrap();
    assert_eq!(false, found);
    assert_eq!(expected, bans);
  }

  #[test]
  fn existing_ban_is_updated_in_file() {
    let mut srcpath = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    srcpath.push("tests/netfilter-data.yaml");
    let mut destpath = env::temp_dir();
    destpath.push("netfilter-data-copy-for-existing.yaml");
    fs::copy(&srcpath, &destpath).unwrap();
    let mut storage = FilesystemNetfilterStorageAdapter {
      file: DataFile::Yaml(destpath.clone().into()),
    };
    let found = storage
      .store_and_remove_obsoletes(
        "ip Inet4 mail_ban",
        "121.66.35.37",
        &Some(DateTime::from(UNIX_EPOCH + Duration::from_secs(2614199400))),
      )
      .unwrap();
    let file = fs::File::open(&destpath).unwrap();
    let bans: Vec<Ban> = Input::Yaml(file).parse().unwrap();
    let expected = vec![
      Ban {
        ip: "59.39.183.34".into(),
        nf_set: "ip Inet4 mail_ban".into(),
        ban_until: Some(DateTime::from(UNIX_EPOCH + Duration::from_secs(2614201649))),
      },
      Ban {
        ip: "51.11.240.49".into(),
        nf_set: "ip Inet4 https_ban".into(),
        ban_until: None,
      },
      Ban {
        ip: "121.66.35.37".into(),
        nf_set: "ip Inet4 mail_ban".into(),
        ban_until: Some(DateTime::from(UNIX_EPOCH + Duration::from_secs(2614199400))),
      },
    ];
    fs::remove_file(&destpath).unwrap();
    assert_eq!(true, found);
    assert_eq!(expected, bans);
  }

  #[test]
  fn default_data_dir_is_in_var() {
    let conf = Config::new(None, None);
    let adapter = FilesystemNetfilterStorageAdapter::new(&conf, "test.yaml");
    assert_eq!(
      DataFile::try_from("/var/lib/pyruse/test.yaml").unwrap(),
      adapter.file
    );
  }

  #[test]
  fn data_dir_is_read_from_config() {
    let mut conf = Config::new(None, None);
    conf
      .options
      .insert("storage".into(), Value::Str("/tmp".into()));
    let adapter = FilesystemNetfilterStorageAdapter::new(&conf, "test.yaml");
    assert_eq!(DataFile::try_from("/tmp/test.yaml").unwrap(), adapter.file);
  }
}
