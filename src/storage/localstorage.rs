use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::{
    image::{IiifImage, ProcessResult},
    storage::Storage,
};

/// LocalStorage 本地存储
///
/// Example:
/// ```
/// use i3f::storage::LocalStorage;
/// use i3f::storage::Storage;
///
/// let storage = LocalStorage::new("./fixtures", "./fixtures/out");
/// ```
pub struct LocalStorage {
    origin_dir: PathBuf,
    iiif_dir: PathBuf,
}

impl Storage for LocalStorage {
    fn get_origin_file(&self, identifier: &str) -> Result<Vec<u8>, String> {
        let path = self.origin_dir.join(identifier);
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        Ok(bytes)
    }

    fn get_iiif_file(&self, params: &IiifImage) -> Result<ProcessResult, String> {
        let iiif_path = params.to_string();
        let path = self.iiif_dir.join(iiif_path);
        println!("iiif_dir: {:?}", self.iiif_dir);
        println!("path: {:?}", path);
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(|e| e.to_string())?;
        Ok(ProcessResult {
            content_type: params.format.get_content_type().to_string(),
            data: bytes,
        })
    }

    fn save_iiif_file(&self, params: &IiifImage, data: &[u8]) -> Result<(), String> {
        let iiif_path = params.to_string();
        let path = self.iiif_dir.join(iiif_path);
        let dir = path.parent();
        if let Some(dir) = dir {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        } else {
            return Err("Failed to get parent directory".to_string());
        }
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(data).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl LocalStorage {
    /// 创建一个新的本地存储实例
    ///
    /// Creates a new local storage instance with the given base path.
    ///
    /// # Arguments
    ///
    /// * `base_path` - The base path for the local storage.
    ///
    /// Example:
    /// ```
    /// use i3f::storage::LocalStorage;
    /// use i3f::storage::Storage;
    ///
    /// let storage = LocalStorage::new("/data/images", "/data/iiif");
    /// ```
    pub fn new<P: AsRef<Path>>(base_dir: P, iiif_dir: P) -> Self {
        Self {
            origin_dir: base_dir.as_ref().to_path_buf(),
            iiif_dir: iiif_dir.as_ref().to_path_buf(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::image::{Format, Quality, Region, Rotation, Size};

    use super::*;

    #[test]
    fn test_get_iiif_file() {
        let storage = LocalStorage::new("./fixtures", "./fixtures/out");
        let origin_file = storage.get_origin_file("demo.jpg").unwrap();
        let image = image::load_from_memory(&origin_file).unwrap();
        assert_eq!(image.width(), 300);
        assert_eq!(image.height(), 200);

        let params = IiifImage {
            identifier: "demo.jpg".to_string(),
            region: Region::Full,
            size: Size::Max,
            rotation: Rotation::Degrees(0.0),
            quality: Quality::Default,
            format: Format::Jpg,
        };
        let result = storage.get_iiif_file(&params).unwrap();
        assert_eq!(result.content_type, "image/jpeg");

        let result = storage.save_iiif_file(&params, &result.data);
        assert!(result.is_ok());
    }
}
