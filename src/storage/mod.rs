mod localstorage;
pub use localstorage::*;

use crate::image::{IiifImage, ProcessResult};

pub trait Storage {
    fn get_origin_file(&self, identifier: &str) -> Result<Vec<u8>, String>;

    fn get_iiif_file(&self, params: &IiifImage) -> Result<ProcessResult, String>;

    fn save_iiif_file(&self, params: &IiifImage, data: &[u8]) -> Result<(), String>;
}
