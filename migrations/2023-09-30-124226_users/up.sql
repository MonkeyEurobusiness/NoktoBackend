-- Your SQL goes here
-- users definition

CREATE TABLE IF NOT EXISTS users (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	username TEXT NOT NULL,
	password TEXT NOT NULL
);
INSERT INTO users
(username, password)
VALUES('user', 'dupa');

create table sessions
(
    user_id    integer     not null,
    token      varchar(32) not null
        constraint sessions_pk
            primary key,
    login_date timestamp default CURRENT_TIMESTAMP not null,
    logout_date timestamp
);

create unique index sessions_token_uindex
    on sessions (token);