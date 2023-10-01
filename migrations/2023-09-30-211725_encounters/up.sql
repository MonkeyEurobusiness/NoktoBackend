-- Your SQL goes here
CREATE TABLE encounters (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    image_urls TEXT,
    created_at TIMESTAMP default CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);