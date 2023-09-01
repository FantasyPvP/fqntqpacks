use std::{fs::{self, OpenOptions}, io::{self, Error, ErrorKind, Write}, path::{Path, PathBuf}};
use crate::IGNORE_FOLDERS_PREFIX;

use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pack_repo: String,
    resourcepacks: String,
    ignore_folder_prefix: String,
}

fn config_path() -> Option<PathBuf> {
    dirs::config_dir()
}

fn create_config_dir(path: &PathBuf) -> Result<(), Error> {
    fs::create_dir_all(path)?;
    create_config_file(&path.join("config.yml"))?;
    Ok(())
}

fn write_config(path: &PathBuf, config: Config) -> Result<(), Error> {
    fs::File::create(path)?;
    Ok(())
}

fn create_config_file(path: &PathBuf) -> Result<(), Error> {
    fs::File::create(path)?;

    let conf_file = Config { 
        pack_repo: ".".to_string(),
        resourcepacks: "./_resourcepacks".to_string(),
        ignore_folder_prefix: "_".to_string(),
    };
    let yaml = serde_yaml::to_string(&conf_file).map_err(|_| Error::new(ErrorKind::Other, "failed to serialise"))?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    file.write_all(yaml.as_bytes())?;
    Ok(())
}

pub fn load_config() -> Result<Config, Error> {
    let mut conf_path = config_path().ok_or(Error::new(ErrorKind::Other, "error getting config path"))?;
    if !conf_path.exists() {
        return Err(Error::new(ErrorKind::Other, "No config directory found"));
    };

    conf_path.push("fqntqpacks");
    if !conf_path.exists() {
        create_config_dir(&conf_path)?;
    };

    conf_path.push("config.yml");
    if !conf_path.exists() {
        create_config_file(&conf_path)?;
    };

    let config: Config = serde_yaml::from_reader(fs::File::open(&conf_path)?)
        .map_err(|_| Error::new(ErrorKind::Other, "failed to deserialise"))?;

    println!(
"-----------------------------------------------------------------
WARNING: no previous config file detected, created config file at 
{}
-----------------------------------------------------------------", conf_path.display());

    Ok(config)
}  