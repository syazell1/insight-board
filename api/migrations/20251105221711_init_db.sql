-- Add migration script here
CREATE TABLE users (
  id uuid NOT NULL,
  PRIMARY KEY(id),
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at timestamptz NOT NULL,
  updated_at timestamptz NULL
);

CREATE TABLE users_tokens (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    token TEXT NOT NULL,
    created_at timestamptz NOT NULL,
    user_id uuid NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);