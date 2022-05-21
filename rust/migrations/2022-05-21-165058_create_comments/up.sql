-- Your SQL goes here
CREATE TABLE comments (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    author VARCHAR NOT NULL,
    body TEXT NOT NULL,
    created_at DATETIME NOT NULL
)