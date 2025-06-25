use std::path::{Path, PathBuf};

pub trait Storage {
    fn get_file_path(&self, identifier: &str) -> PathBuf;
}

/// LocalStorage 本地存储
///
/// Example:
/// ```
/// use iiif::LocalStorage;
///
/// let storage = LocalStorage::new("/data/images");
/// assert_eq!(storage.get_file_path("1234567890"), "/data/images/1234567890");
/// ```
pub struct LocalStorage {
    base_path: PathBuf,
}

impl Storage for LocalStorage {
    fn get_file_path(&self, identifier: &str) -> PathBuf {
        self.base_path.join(identifier)
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
    /// use iiif::LocalStorage;
    ///
    /// let storage = LocalStorage::new("/data/images");
    /// assert_eq!(storage.get_file_path("1234567890"), "/data/images/1234567890");
    /// ```
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_file_path() {
        let storage = LocalStorage::new("./fixtures");
        assert_eq!(
            storage.get_file_path("image3.png"),
            PathBuf::from("./fixtures/image3.png"),
        );
    }
}
