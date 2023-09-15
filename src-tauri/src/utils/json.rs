use std::{fs, path::PathBuf};

use anyhow::{bail, Context, Result};
use log::debug;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

pub fn stringify<T: Serialize>(data: &T) -> Result<String> {
    Ok(serde_json::to_string(data)?)
}

pub fn parse<T: DeserializeOwned>(json_str: &str) -> Result<T> {
    serde_json::from_str::<T>(json_str).context("failed to parse json string")
}

pub fn read<T: DeserializeOwned>(path: &PathBuf) -> Result<T> {
    if !path.exists() {
        bail!("file not found \"{}\"", path.display());
    }

    let json_str = fs::read_to_string(path)
        .context(format!("failed to read the file \"{}\"", path.display()))?;

    serde_json::from_str::<T>(&json_str).context(format!(
        "failed to read the file with json format \"{}\"",
        path.display()
    ))
}

pub fn save<T: Serialize>(path: &PathBuf, data: &T) -> Result<()> {
    let data_str = serde_json::to_string(data)?;
    debug!("data_str: {}", data_str);
    let path_str = path.as_os_str().to_string_lossy().to_string();
    fs::write(path, data_str.as_bytes()).context(format!("failed to save file \"{path_str}\""))
}
