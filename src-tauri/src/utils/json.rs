use anyhow::Result;
use serde::Serialize;

pub fn stringify<T: Serialize>(data: &T) -> Result<String> {
    Ok(serde_json::to_string(data)?)
}
