-- Your SQL goes here
CREATE TABLE service_quotas (
    id INTEGER PRIMARY KEY,
    telegram_id INTEGER NOT NULL UNIQUE,
    cpu INTEGER NOT NULL,
    memory INTEGER NOT NULL,
    disk_size INTEGER NOT NULL
);