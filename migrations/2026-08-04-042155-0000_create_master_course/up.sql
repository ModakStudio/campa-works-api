CREATE TYPE course_type AS ENUM (
    'UNDERGRADUATE',
    'GRADUATE'
);

CREATE TABLE master_course (
    id BIGSERIAL PRIMARY KEY,

    course_code VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,

    credit INT NOT NULL,
    lecture INT NOT NULL,
    practice INT NOT NULL,

    course_type course_type NOT NULL,

    is_core BOOLEAN NOT NULL
);
