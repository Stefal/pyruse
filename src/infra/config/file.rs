use super::SerdeConfigAdapter;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

const ENV_VARIABLE: &'static str = "PYRUSE_CONF";
const ETC_PATH: &'static str = "/etc/pyruse";

pub enum ConfFile {
  Json(OsString),
  Yaml(OsString),
}

impl ConfFile {
  pub fn from_filesystem() -> ConfFile {
    find_file(find_candidates())
  }

  pub fn to_config(self) -> SerdeConfigAdapter {
    match self {
      ConfFile::Json(path) => {
        SerdeConfigAdapter::from_json(BufReader::new(File::open(path).expect("Read error")))
      }
      ConfFile::Yaml(path) => {
        SerdeConfigAdapter::from_yaml(BufReader::new(File::open(path).expect("Read error")))
      }
    }
  }
}

fn find_candidates() -> Vec<ConfFile> {
  match env::var_os(ENV_VARIABLE) {
    Some(path) => {
      let s = Path::new(&path)
        .extension()
        .and_then(|e| Some(e.to_string_lossy()))
        .and_then(|s| Some(s.to_ascii_lowercase()))
        .unwrap_or_default();
      match s.as_ref() {
        "json" => vec![ConfFile::Json(path)],
        "yaml" | "yml" => vec![ConfFile::Yaml(path)],
        _ => panic!(
          "Cannot determine file format from file name: {}",
          path.to_string_lossy()
        ),
      }
    }
    None => {
      let cwd = env::current_dir().expect("Error accessing the current working directory");
      let add_file: fn(&PathBuf, &str) -> OsString = |c, f| {
        let mut c2 = c.clone();
        c2.push(f); // not a fluent API…
        c2.into_os_string()
      };
      vec![
        ConfFile::Json(add_file(&cwd, "pyruse.yml")),
        ConfFile::Yaml(add_file(&cwd, "pyruse.yaml")),
        ConfFile::Yaml(add_file(&cwd, "pyruse.yml")),
        ConfFile::Json(OsString::from(format!("{}/{}", ETC_PATH, "pyruse.json"))),
        ConfFile::Yaml(OsString::from(format!("{}/{}", ETC_PATH, "pyruse.yaml"))),
        ConfFile::Yaml(OsString::from(format!("{}/{}", ETC_PATH, "pyruse.yml"))),
      ]
    }
  }
}

fn find_file(conf_candidates: Vec<ConfFile>) -> ConfFile {
  for name in conf_candidates {
    match name {
      ConfFile::Json(ref path) | ConfFile::Yaml(ref path) => {
        if Path::new(&path).exists() {
          return name;
        }
      }
    }
  }
  panic!("No configuration found. Consider setting ${}, or creating one of these in $PWD: pyruse.json, pyruse.yaml or pyruse.yml", ENV_VARIABLE)
}
