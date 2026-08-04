CREATE TYPE course_category AS ENUM (
    'MAJOR_REQUIRED',
    'MAJOR_ELECTIVE',
    'GENERAL_REQUIRED',
    'GENERAL_ELECTIVE'
);

CREATE TYPE language AS ENUM (
    'KOREAN',
    'ENGLISH'
);

CREATE TABLE course (
    id BIGSERIAL PRIMARY KEY,

    master_course_id BIGINT NOT NULL,
    semester_id BIGINT NOT NULL,
    major_id BIGINT NOT NULL,

    description VARCHAR(500),

    course_category course_category NOT NULL,

    language language NOT NULL,

    section_number INT NOT NULL,
    grade INT NOT NULL,
    capacity INT NOT NULL,

    CONSTRAINT fk_course_master_course
        FOREIGN KEY(master_course_id)
        REFERENCES master_course(id),

    CONSTRAINT fk_course_semester
        FOREIGN KEY(semester_id)
        REFERENCES semester(id),

    CONSTRAINT fk_course_major
        FOREIGN KEY(major_id)
        REFERENCES major(id)
);

CREATE UNIQUE INDEX idx_course_unique
ON course(
    semester_id,
    master_course_id,
    section_number
);
