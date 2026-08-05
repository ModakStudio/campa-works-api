CREATE TABLE course_preference_bookmark (
    id BIGSERIAL PRIMARY KEY,

    professor_id BIGINT NOT NULL,
    master_course_id BIGINT NOT NULL,

    CONSTRAINT fk_course_preference_bookmark_professor
        FOREIGN KEY (professor_id)
        REFERENCES professor(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_course_preference_bookmark_master_course
        FOREIGN KEY (master_course_id)
        REFERENCES master_course(id)
        ON DELETE CASCADE,

    CONSTRAINT uq_course_preference_bookmark
        UNIQUE (professor_id, master_course_id)
);
