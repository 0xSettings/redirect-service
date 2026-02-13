CREATE TABLE IF NOT EXISTS urls (
    short_key    VARCHAR(20)  PRIMARY KEY,
    original_url TEXT         NOT NULL UNIQUE,
    created_at   TIMESTAMP    NOT NULL DEFAULT NOW()
);