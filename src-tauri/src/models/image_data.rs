use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ImageData {
    pub width: usize,
    pub height: usize,
    pub base64: String,
}
