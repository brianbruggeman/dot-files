use std::borrow::Cow;
use std::path::{Path, PathBuf};

use path_absolutize::Absolutize;


pub fn true_path(path: impl AsRef<Path>) -> PathBuf {
    fn inner(path: &Path) -> PathBuf {
        // Expand `~` to home directory
        let tilde_expanded = match path.to_str() {
            Some(str_path) => match shellexpand::tilde(str_path) {
                Cow::Borrowed(s) => Cow::Borrowed(Path::new(s)),
                Cow::Owned(s) => Cow::Owned(PathBuf::from(s)),
            },
            None => Cow::Borrowed(path),
        };

        // Get absolute path
        let abs_path = match tilde_expanded.absolutize() {
            Ok(path) => path,
            Err(_) => tilde_expanded,
        };

        // Canonicalize to resolve all symbolic links if it exists
        match abs_path.canonicalize() {
            Ok(path) => path,
            Err(_) => abs_path.to_path_buf(),
        }
    }
    inner(path.as_ref())
}