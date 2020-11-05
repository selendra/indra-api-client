-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    login_session VARCHAR NOT NULL DEFAULT ''
);