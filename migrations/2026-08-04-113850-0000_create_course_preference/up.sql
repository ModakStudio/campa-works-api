CREATE TABLE course_preference (
    id BIGSERIAL PRIMARY KEY,

    semester_id BIGINT NOT NULL,
    professor_id BIGINT NOT NULL,
    master_course_id BIGINT NOT NULL,

    priority INT NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_course_preference_semester
        FOREIGN KEY (semester_id)
        REFERENCES semester(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_course_preference_professor
        FOREIGN KEY (professor_id)
        REFERENCES professor(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_course_preference_master_course
        FOREIGN KEY (master_course_id)
        REFERENCES master_course(id)
        ON DELETE CASCADE,

    CONSTRAINT uq_course_preference_priority
        UNIQUE (
            semester_id,
            professor_id,
            priority
        ),

    CONSTRAINT uq_course_preference_course
        UNIQUE (
            semester_id,
            professor_id,
            master_course_id
        )
);
