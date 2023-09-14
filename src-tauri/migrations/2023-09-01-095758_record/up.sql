-- Your SQL goes here
create table if not exists t_record
(
    id              INTEGER             NOT NULL PRIMARY KEY AUTOINCREMENT,
    content         TEXT                NOT NULL DEFAULT '',
    content_preview TEXT,
    data_type       VARCHAR(20)         NOT NULL DEFAULT '',
    md5             VARCHAR(200) UNIQUE NOT NULL DEFAULT '',
    create_time     INTEGER             NOT NULL DEFAULT 0,
    is_favorite     INTEGER             NOT NULL DEFAULT 0,
    tags            VARCHAR(256)        NOT NULL DEFAULT '',
    latest_addr     VARCHAR(256)        NOT NULL DEFAULT '127.0.0.1'
);
