use std::env;
use std::ffi::{OsString};
use std::io::{BufReader,Error,Result};
use std::path::Path;
use std::fs::File;

const ENV_VARIABLE: &'static str = "PYRUSE_CONF";

enum ConfFile {
  Json(OsString),
  Yaml(OsString)
}

pub fn from_file() {
  match find_file(find_candidates()) {
    ConfFile::Json(path) => super::parse_json(BufReader::new(File::open(path).expect("Read error"))),
    ConfFile::Yaml(path) => super::parse_yaml(BufReader::new(File::open(path).expect("Read error")))
  };
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
        _ => panic!("Cannot determine file format from file name: {}", path.to_string_lossy())
      }
    },
    None => {
      vec![
        ConfFile::Json(OsString::from("pyruse.json")),
        ConfFile::Yaml(OsString::from("pyruse.yaml")),
        ConfFile::Yaml(OsString::from("pyruse.yml"))
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
