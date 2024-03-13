-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  status VARCHAR NOT NULL DEFAULT 'pending'
);

CREATE UNIQUE INDEX users_email_index ON users (email) where status = 'active';
