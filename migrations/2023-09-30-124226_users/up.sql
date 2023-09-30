-- Your SQL goes here
-- Users definition

CREATE TABLE IF NOT EXISTS Users (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	username TEXT NOT NULL,
	password TEXT NOT NULL
);