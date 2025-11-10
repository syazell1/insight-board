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

CREATE TABLE api_endpoints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    interval_seconds INT NOT NULL DEFAULT 60, 
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    last_checked_at TIMESTAMP WITH TIME ZONE,
    last_status_code INT,
    last_latency_ms INT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE api_metrics (
    id BIGSERIAL PRIMARY KEY,
    api_id UUID NOT NULL REFERENCES api_endpoints(id) ON DELETE CASCADE,
    checked_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    status_code INT,
    latency_ms INT,
    is_success BOOLEAN,
    error_message TEXT
);
