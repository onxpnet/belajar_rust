-- Add up migration script here

CREATE TABLE IF NOT EXISTS actix_user (
  id int PRIMARY KEY NOT NULL,
  name varchar NOT NULL,
  email varchar NOT NULL
);
