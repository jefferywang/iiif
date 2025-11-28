mod localstorage;
pub use localstorage::*;

use std::path::PathBuf;

pub trait Storage {
    fn get_file_path(&self, identifier: &str) -> PathBuf;
}
