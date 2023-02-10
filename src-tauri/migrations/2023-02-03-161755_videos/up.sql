-- Your SQL goes here
CREATE TABLE video(
    id integer not null primary key,
    title varchar not null,
    description text not null,
    removed boolean not null
);