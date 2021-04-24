use crate::domain::Error;
use crate::infra::serde::{Input, Output};
use std::{
  convert::TryFrom,
  ffi::OsString,
  fs::{File, OpenOptions},
  io::BufReader,
  path::{Path, PathBuf},
  str::FromStr,
};

#[derive(Debug, PartialEq, Eq)]
pub enum DataFile {
  Json(OsString),
  Yaml(OsString),
}
impl DataFile {
  pub fn exists(&self) -> bool {
    match &self {
      DataFile::Json(ref path) | DataFile::Yaml(ref path) => Path::new(path).exists(),
    }
  }

  pub fn open_r(&self) -> Result<Input<BufReader<File>>, Error> {
    Ok(match self {
      DataFile::Json(path) => Input::Json(BufReader::new(File::open(path)?)),
      DataFile::Yaml(path) => Input::Yaml(BufReader::new(File::open(path)?)),
    })
  }

  pub fn open_w(&self) -> Result<Output<File>, Error> {
    let mut opt = OpenOptions::new();
    let wopt = opt.create(true).write(true).truncate(true);
    Ok(match self {
      DataFile::Json(path) => Output::Json(wopt.open(path)?),
      DataFile::Yaml(path) => Output::Yaml(wopt.open(path)?),
    })
  }
}

impl TryFrom<(&PathBuf, &str)> for DataFile {
  type Error = Error;
  fn try_from((parent, file): (&PathBuf, &str)) -> Result<Self, Error> {
    let mut o = parent.clone();
    o.push(file);
    DataFile::try_from(o.into_os_string())
  }
}

impl TryFrom<&str> for DataFile {
  type Error = Error;
  fn try_from(path: &str) -> Result<Self, Error> {
    DataFile::try_from(OsString::from_str(path)?)
  }
}

impl TryFrom<OsString> for DataFile {
  type Error = Error;
  fn try_from(path: OsString) -> Result<Self, Error> {
    let ext = Path::new(&path)
      .extension()
      .and_then(|e| Some(e.to_string_lossy()))
      .and_then(|s| Some(s.to_ascii_lowercase()))
      .unwrap_or_default();
    match ext.as_ref() {
      "json" => Ok(DataFile::Json(path)),
      "yaml" | "yml" => Ok(DataFile::Yaml(path)),
      _ => Err(format!("File {:?} does not have extension .json or .yaml", path).into()),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::infra::file::DataFile;
  use std::{convert::TryFrom, env, ffi::OsString, fs, path::PathBuf, str::FromStr};

  #[test]
  fn json_path_becomes_a_json_variant() {
    if let Ok(DataFile::Json(_)) = DataFile::try_from("/Some/File name.Json") {
      ()
    } else {
      panic!("Expected Json variant")
    }
  }

  #[test]
  fn yaml_path_becomes_a_yaml_variant() {
    if let Ok(DataFile::Yaml(_)) = DataFile::try_from("/Some/File name.Yaml") {
      ()
    } else {
      panic!("Expected Yaml variant")
    }
  }

  #[test]
  fn yml_path_becomes_a_yaml_variant() {
    if let Ok(DataFile::Yaml(_)) = DataFile::try_from("/Some/File name.Yml") {
      ()
    } else {
      panic!("Expected Yaml variant")
    }
  }

  #[test]
  fn when_neither_yaml_nor_json_then_unknown_file() {
    if let Ok(_) = DataFile::try_from("/Some/File name") {
      panic!("Expected an error")
    }
  }

  #[test]
  fn pathbuf_and_str_can_be_combined_and_converted() {
    let parent = PathBuf::from("/It’s");
    let files = (
      DataFile::try_from((&parent, "a.json")),
      DataFile::try_from((&parent, "a.yaml")),
    );
    assert_eq!(
      (
        Ok(DataFile::Json(OsString::from_str("/It’s/a.json").unwrap())),
        Ok(DataFile::Yaml(OsString::from_str("/It’s/a.yaml").unwrap()))
      ),
      files
    );
  }

  #[test]
  fn exists_returns_true_for_existing_file() {
    let tempdir = env::temp_dir();
    let tempfile = tempdir.join("test-exists.yaml");
    let filename = tempfile.to_str().unwrap();
    let datafile = DataFile::try_from(filename).unwrap();
    fs::File::create(filename).unwrap();
    let exists = datafile.exists();
    fs::remove_file(filename).unwrap();
    assert_eq!(true, exists);
  }

  #[test]
  fn exists_returns_false_for_absent_file() {
    let filename = "→ I don’t exist 😱 ←.yaml";
    let datafile = DataFile::try_from(filename).unwrap();
    let exists = datafile.exists();
    assert_eq!(false, exists);
  }
}
