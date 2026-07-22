-- ===========================
-- ENUM
-- ===========================

CREATE TYPE user_role AS ENUM (
    'ADMIN',
    'PROFESSOR',
    'ASSISTANT'
);

CREATE TYPE user_status AS ENUM (
    'ACTIVE',
    'INACTIVE'
);


-- ===========================
-- TABLE : users
-- ===========================

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,

    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    name VARCHAR(100) NOT NULL,

    role user_role NOT NULL,
    status user_status NOT NULL DEFAULT 'ACTIVE',

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


-- ===========================
-- INDEX
-- ===========================

CREATE INDEX idx_users_email
ON users(email);
