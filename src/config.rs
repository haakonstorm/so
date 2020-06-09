use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use crate::error::{Error, PermissionType, Result};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub api_key: Option<String>,
    pub limit: u16,
    pub site: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_key: None,
            limit: 20,
            site: String::from("stackoverflow"),
        }
    }
}

/// Get user config (writes default if none found)
pub fn user_config() -> Result<Config> {
    let project = project_dir()?;
    let dir = project.config_dir();
    fs::create_dir_all(&dir)
        .map_err(|_| Error::Permissions(PermissionType::Create, dir.to_path_buf()))?;
    let filename = config_file_name()?;
    match File::open(&filename) {
        Err(_) => {
            let def = Config::default();
            write_config(&def)?;
            Ok(def)
        }
        Ok(file) => serde_yaml::from_reader(file).map_err(|_| Error::MalformedFile(filename)),
    }
}

fn write_config(config: &Config) -> Result<()> {
    let filename = config_file_name()?;
    let file = File::create(&filename)
        .map_err(|_| Error::Permissions(PermissionType::Create, filename.clone()))?;
    serde_yaml::to_writer(file, config)
        .map_err(|_| Error::Permissions(PermissionType::Write, filename.clone()))
}

fn config_file_name() -> Result<PathBuf> {
    Ok(project_dir()?.config_dir().join("config.yml"))
}

/// Get project directory
pub fn project_dir() -> Result<ProjectDirs> {
    ProjectDirs::from("io", "Sam Tay", "so").ok_or_else(|| Error::ProjectDir)
}

pub fn set_api_key(key: String) -> Result<()> {
    let mut cfg = user_config()?;
    cfg.api_key = Some(key);
    write_config(&cfg)
}
