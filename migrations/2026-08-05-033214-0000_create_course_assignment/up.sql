CREATE TABLE course_assignment (
    id BIGSERIAL PRIMARY KEY,

    course_id BIGINT NOT NULL UNIQUE,
    professor_id BIGINT NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_course_assignment_course
        FOREIGN KEY (course_id)
        REFERENCES course(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_course_assignment_professor
        FOREIGN KEY (professor_id)
        REFERENCES professor(id)
        ON DELETE CASCADE
);
