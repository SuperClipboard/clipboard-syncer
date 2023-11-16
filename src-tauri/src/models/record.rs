pub const SCHEMA_ID: &str =
    "record_002017915c937c1c44d1d6a7bc6697b2760396843676cc418a02b481fb08009e099f";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Record {
    pub content: String,
    // 文字为空，图片为缩略图
    pub content_preview: Option<String>,
    // data_type(文本=text、图片=image)
    pub data_type: String,
    pub md5: String,
    pub create_time: i32,
    pub is_favorite: i32,
    pub tags: String,
    pub latest_addr: String,
    pub is_deleted: i32,
}

pub enum DataTypeEnum {
    TEXT,
    IMAGE,
}

impl From<DataTypeEnum> for String {
    fn from(value: DataTypeEnum) -> Self {
        match value {
            DataTypeEnum::TEXT => "text".into(),
            DataTypeEnum::IMAGE => "image".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_build() {}
}
