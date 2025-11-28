use std::path::{Path, PathBuf};

use crate::storage::Storage;

/// LocalStorage 本地存储
///
/// Example:
/// ```
/// use i3f::storage::LocalStorage;
/// use i3f::storage::Storage;
///
/// let storage = LocalStorage::new("./fixtures");
/// println!("{:?}", storage.get_file_path("demo.jpg"));
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
    /// use i3f::storage::LocalStorage;
    /// use i3f::storage::Storage;
    ///
    /// let storage = LocalStorage::new("/data/images");
    /// println!("{:?}", storage.get_file_path("1234567890"));
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
