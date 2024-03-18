use std::path::{Path, PathBuf};
use home::home_dir;

/// Configuration for My Dotfiles Manager
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MdmConfig {
    /// Path to the folder containing the dotfiles
    dots_path: PathBuf,

    /// Path to the local repository containing the dotfiles
    repo_path: PathBuf,

    /// Private/Ignored fields
    ignored_paths: Vec<PathBuf>,
}

impl MdmConfig {
    pub fn dots_path(&self) -> &PathBuf {
        &self.dots_path
    }

    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_path
    }

    pub fn ignored_paths(&self) -> &Vec<PathBuf> {
        &self.ignored_paths
    }

    pub fn builder() -> MdmConfigBuilder {
        MdmConfigBuilder::from_config(MdmConfig::default())
    }

    pub fn as_builder(self) -> MdmConfigBuilder {
        MdmConfigBuilder::from_config(self)
    }

    pub fn from_builder(builder: MdmConfigBuilder) -> Self {
        builder.as_config()
    }
}


impl Default for MdmConfig {
    fn default() -> Self {
        let home_dir = home_dir().expect("Unable to determine home directory");
        let dots_path = home_dir.join("repos/mine/dot-files/dots");
        let repo_path = home_dir.join("repos/mine/dot-files");
        let ignored_paths: Vec<PathBuf> = include_str!("../.gitignore")
            .lines()
            .filter(|line| line.starts_with('.') || line.starts_with("**/."))
            .map(PathBuf::from)
            .collect();
        MdmConfig { dots_path, repo_path, ignored_paths }
    }
}

pub struct MdmConfigBuilder {
    config: MdmConfig,
}

impl MdmConfigBuilder {
    pub fn new() -> Self {
        MdmConfigBuilder { config: MdmConfig::default() }
    }

    pub fn dots_path(mut self, dots_path: impl AsRef<Path>) -> Self {
        self.config.dots_path = dots_path.as_ref().to_owned();
        self
    }

    pub fn repo_path(mut self, repo_path: impl AsRef<Path>) -> Self {
        self.config.repo_path = repo_path.as_ref().to_owned();
        self
    }

    pub fn add_ignore_path(mut self, path: impl AsRef<Path>) -> Self {
        self.config.ignored_paths.push(path.as_ref().to_owned());
        self
    }

    pub fn remove_ignore_path(mut self, path: impl AsRef<Path>) -> Self {
        self.config.ignored_paths.retain(|p| p != path.as_ref());
        self
    }

    pub fn extend_ignore_paths(mut self, paths: &[impl AsRef<Path>]) -> Self {
        self.config.ignored_paths.extend(paths.iter().map(|p| p.as_ref().to_owned()));
        self
    }

    pub fn as_config(self) -> MdmConfig {
        self.config
    }

    pub fn from_config(config: MdmConfig) -> Self {
        MdmConfigBuilder { config }
    }
}