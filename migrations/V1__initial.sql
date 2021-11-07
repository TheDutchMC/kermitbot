CREATE TABLE uwu_counter (
    user_id BIGINT PRIMARY KEY NOT NULL,
    count INT NOT NULL DEFAULT 1
);

CREATE TABLE user_nicknames (
    user_id BIGINT PRIMARY KEY NOT NULL,
    nickname VARCHAR(32) NOT NULL
)