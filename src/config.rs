use std::collections::HashMap;
use std::path::{Path, PathBuf};

use home::home_dir;


#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MdmProfile {
    /// Path to the folder containing the dotfiles
    dots_path: PathBuf,

    /// Path to the local repository containing the dotfiles
    repo_path: PathBuf,

    /// Private/Ignored fields
    ignored_paths: Vec<PathBuf>,
}

impl Default for MdmProfile {
    fn default() -> Self {
        let home_dir = match home_dir() {
            Some(home_dir) => home_dir,
            None => panic!("Could not find home directory"),
        };
        let dots_path = home_dir.join("repos/mine/dot-files/dots");
        let repo_path = home_dir.join("repos/mine/dot-files");
        let ignored_paths: Vec<PathBuf> = include_str!("../.gitignore")
            .lines()
            .filter(|line| line.starts_with('.') || line.starts_with("**/."))
            .map(PathBuf::from)
            .collect();
        MdmProfile { dots_path, repo_path, ignored_paths }
    }
}

impl MdmProfile {
    pub fn dots_path(&self) -> &PathBuf {
        &self.dots_path
    }

    pub fn with_dots_path(&mut self, dots_path: impl AsRef<Path>) -> &mut Self {
        self.dots_path = dots_path.as_ref().to_owned();
        self
    }

    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_path
    }

    pub fn with_repo_path(&mut self, repo_path: impl AsRef<Path>) -> &mut Self {
        self.repo_path = repo_path.as_ref().to_owned();
        self
    }

    pub fn ignored_paths(&self) -> &Vec<PathBuf> {
        &self.ignored_paths
    }

    pub fn add_ignored_path(&mut self, path: impl AsRef<Path>) -> &mut Self {
        let path = path.as_ref().to_owned();
        if !self.ignored_paths.contains(&path) {
            self.ignored_paths.push(path);
        }
        self
    }

    pub fn builder() -> MdmProfileBuilder {
        MdmProfileBuilder::from_config(MdmProfile::default())
    }

    pub fn as_builder(self) -> MdmProfileBuilder {
        MdmProfileBuilder::from_config(self)
    }

    pub fn from_builder(builder: MdmProfileBuilder) -> Self {
        builder.as_config()
    }
}


pub struct MdmProfileBuilder {
    profile: MdmProfile,
}

impl MdmProfileBuilder {
    pub fn new() -> Self {
        MdmProfileBuilder { profile: MdmProfile::default() }
    }

    pub fn dots_path(mut self, dots_path: impl AsRef<Path>) -> Self {
        self.profile.dots_path = dots_path.as_ref().to_owned();
        self
    }

    pub fn repo_path(mut self, repo_path: impl AsRef<Path>) -> Self {
        self.profile.repo_path = repo_path.as_ref().to_owned();
        self
    }

    pub fn add_ignore_path(mut self, path: impl AsRef<Path>) -> Self {
        self.profile.ignored_paths.push(path.as_ref().to_owned());
        self
    }

    pub fn remove_ignore_path(mut self, path: impl AsRef<Path>) -> Self {
        self.profile.ignored_paths.retain(|p| p != path.as_ref());
        self
    }

    pub fn extend_ignore_paths(mut self, paths: &[impl AsRef<Path>]) -> Self {
        self.profile.ignored_paths.extend(paths.iter().map(|p| p.as_ref().to_owned()));
        self
    }

    pub fn as_config(self) -> MdmProfile {
        self.profile
    }

    pub fn from_config(config: MdmProfile) -> Self {
        MdmProfileBuilder { profile: config }
    }
}

/// Configuration for My Dotfiles Manager
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MdmConfig {
    /// Path to the folder containing the dotfiles
    profiles: HashMap<String, MdmProfile>,
}

impl Default for MdmConfig {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        profiles.insert("default".to_string(), MdmProfile::default());
        MdmConfig { profiles }
    }
}

impl MdmConfig {
    pub fn profiles(&self) -> &HashMap<String, MdmProfile> {
        &self.profiles
    }

    pub fn add_profile(mut self, name: impl AsRef<str>, profile: MdmProfile) -> Self {
        self.profiles.insert(name.as_ref().to_string(), profile);
        self
    }

    pub fn remove_profile(mut self, name: impl AsRef<str>) -> Self {
        self.profiles.remove(name.as_ref());
        self
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<&MdmProfile> {
        self.profiles.get(name.as_ref())
    }

    pub fn get_mut(&mut self, name: impl AsRef<str>) -> Option<&mut MdmProfile> {
        self.profiles.get_mut(name.as_ref())
    }
}