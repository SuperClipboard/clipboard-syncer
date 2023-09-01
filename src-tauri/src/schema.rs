// @generated automatically by Diesel CLI.

diesel::table! {
    t_record (id) {
        id -> Integer,
        content -> Text,
        content_preview -> Nullable<Text>,
        data_type -> Text,
        md5 -> Text,
        create_time -> Integer,
        is_favorite -> Integer,
        tags -> Text,
    }
}
