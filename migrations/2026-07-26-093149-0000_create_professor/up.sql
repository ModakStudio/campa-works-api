-- ===========================
-- ENUM
-- ===========================

CREATE TYPE professor_position AS ENUM (
    'PROFESSOR',
    'INVITED',
    'CONCURRENT',
    'VISITING',
    'EMERITUS'
);

CREATE TYPE professor_status AS ENUM (
    'ACTIVE',
    'INACTIVE'
);


-- ===========================
-- TABLE : users
-- ===========================

CREATE TABLE professor (
    id BIGSERIAL PRIMARY KEY,

    user_id BIGINT NOT NULL UNIQUE,

    position professor_position NOT NULL,

    office VARCHAR(255),
    tel VARCHAR(50),
    research_field VARCHAR(255),

    status professor_status NOT NULL DEFAULT 'ACTIVE',

    CONSTRAINT fk_professor_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);