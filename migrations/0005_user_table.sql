-- migrate:up
CREATE TABLE users
(
    id       SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    password TEXT NOT NULL,
    email    TEXT NOT NULL
);

-- migrate:down
DROP TABLE IF EXISTS users;