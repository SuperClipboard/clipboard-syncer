use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use log::debug;
use p2panda_rs::identity::KeyPair;

use crate::utils::dir::secret_path;

/// Get or generate the key pair
pub fn get_key_pair() -> Result<KeyPair> {
    let secret_path = secret_path().unwrap();

    // Read private key from file or generate a new one
    let private_key = if Path::exists(&secret_path) {
        let key = read_to_string(secret_path)?;
        debug!("Load private key from file success!");
        key.replace('\n', "")
    } else {
        let key = hex::encode(KeyPair::new().private_key().to_bytes());
        let mut file = File::create(secret_path)?;
        write!(&mut file, "{}", &key).unwrap();
        debug!("Generate new private key success!");
        key
    };

    // Derive key pair from private key
    Ok(KeyPair::from_private_key_str(&private_key)?)
}
