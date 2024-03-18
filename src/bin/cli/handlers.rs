use std::path::Path;

// Calls the library handler for symlinking the dot files in the `dots` folder to the home folder
pub async fn link_dotfiles(path: impl AsRef<Path>, force: bool, ignored_paths: &[impl AsRef<Path>]) -> anyhow::Result<()> {
    mdm::symlink_dotfiles(path, force, ignored_paths).await
}

// Syncs the local dot-files repository to the remote host (github)
pub async fn sync_dotfiles(path: impl AsRef<Path>) -> anyhow::Result<()> {
    mdm::sync_github(path).await
}
