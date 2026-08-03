-- ===========================
-- ENUM
-- ===========================

CREATE TYPE semester_type AS ENUM (
    'FIRST',
    'SUMMER',
    'SECOND',
    'WINTER'
);

CREATE TYPE semester_status AS ENUM (
    'OPEN',
    'CLOSED'
);

-- ===========================
-- TABLE : semester
-- ===========================

CREATE TABLE semester (
    id BIGSERIAL PRIMARY KEY,

    year INTEGER NOT NULL,
    semester semester_type NOT NULL,

    status semester_status NOT NULL DEFAULT 'OPEN',

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    UNIQUE (year, semester)
);
