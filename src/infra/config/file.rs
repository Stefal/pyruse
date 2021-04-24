use crate::infra::file::DataFile;
use std::convert::TryFrom;
use std::env;

const ENV_VARIABLE: &'static str = "PYRUSE_CONF";
pub const ETC_PATH: &'static str = "/etc/pyruse";

pub fn configuration_from_filesystem(conf_dir: &str) -> DataFile {
  find_file(find_candidates(conf_dir))
}

fn find_candidates(conf_dir: &str) -> Vec<DataFile> {
  match env::var_os(ENV_VARIABLE) {
    Some(path) => vec![DataFile::try_from(path).unwrap()],
    None => {
      let cwd = env::current_dir().expect("Error accessing the current working directory");
      vec![
        DataFile::try_from((&cwd, "pyruse.json")).unwrap(),
        DataFile::try_from((&cwd, "pyruse.yaml")).unwrap(),
        DataFile::try_from((&cwd, "pyruse.yml")).unwrap(),
        DataFile::try_from(format!("{}/{}", conf_dir, "pyruse.json").as_ref()).unwrap(),
        DataFile::try_from(format!("{}/{}", conf_dir, "pyruse.yaml").as_ref()).unwrap(),
        DataFile::try_from(format!("{}/{}", conf_dir, "pyruse.yml").as_ref()).unwrap(),
      ]
    }
  }
}

fn find_file(conf_candidates: Vec<DataFile>) -> DataFile {
  for name in conf_candidates {
    if name.exists() {
      return name;
    }
  }
  panic!("No configuration found. Consider setting ${}, or creating one of these in $PWD: pyruse.json, pyruse.yaml or pyruse.yml", ENV_VARIABLE)
}

#[macro_use]
#[cfg(test)]
mod tests {
  use super::{configuration_from_filesystem, ENV_VARIABLE};
  use crate::infra::file::DataFile;
  use crate::test_with_exclusive_env_access;
  use lazy_static::lazy_static;
  use std::convert::TryFrom;
  use std::env;
  use std::fs;
  use std::panic;
  use std::sync::Mutex;

  lazy_static! {
    static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
  }

  test_with_exclusive_env_access! {
    fn if_envvar_is_set_then_the_configuration_comes_from_there() {
      let mut temp = env::temp_dir();
      temp.push("pyruse-conf.yaml");
      let filename = temp.to_str().unwrap();
      env::set_var(ENV_VARIABLE, filename);
      fs::File::create(filename).unwrap();
      assert_eq!(
        DataFile::try_from(filename).unwrap(),
        configuration_from_filesystem("")
      );
      fs::remove_file(filename).unwrap();
    }
  }

  test_with_exclusive_env_access! {
    fn if_envvar_is_unset_then_the_configuration_is_found_in_search_dir() {
      env::set_current_dir("/").unwrap();
      env::remove_var(ENV_VARIABLE);
      let tempdir = env::temp_dir();
      let tempfile = tempdir.join("pyruse.yml");
      let dirname = tempdir.to_str().unwrap();
      let filename = tempfile.to_str().unwrap();
      fs::File::create(filename).unwrap();
      assert_eq!(
        DataFile::try_from(filename).unwrap(),
        configuration_from_filesystem(dirname)
      );
      fs::remove_file(filename).unwrap();
    }
  }

  // https://github.com/rust-lang/rust/issues/43155#issuecomment-315543432
  // Environment variables and current working directory are for all tests “simultaneously”.
  #[macro_export]
  macro_rules! test_with_exclusive_env_access {
    (fn $name:ident() $body:block) => {
      #[test]
      fn $name() {
        let guard = ENV_MUTEX.lock().unwrap();
        if let Err(e) = panic::catch_unwind(|| $body) {
          drop(guard);
          panic::resume_unwind(e);
        }
      }
    };
  }
}
