CREATE TABLE course_pool (
    id BIGSERIAL PRIMARY KEY,

    professor_id BIGINT NOT NULL,
    master_course_id BIGINT NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_course_pool_professor
        FOREIGN KEY (professor_id)
        REFERENCES professor(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_course_pool_master_course
        FOREIGN KEY (master_course_id)
        REFERENCES master_course(id)
        ON DELETE CASCADE,

    CONSTRAINT uq_course_pool
        UNIQUE(professor_id, master_course_id)
);
