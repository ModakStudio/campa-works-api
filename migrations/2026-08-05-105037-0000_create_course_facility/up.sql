CREATE TABLE course_facility (
    id BIGSERIAL PRIMARY KEY,

    master_course_id BIGINT NOT NULL,
    facility_id BIGINT NOT NULL,

    CONSTRAINT fk_course_facility_master_course
        FOREIGN KEY (master_course_id)
        REFERENCES master_course(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_course_facility_facility
        FOREIGN KEY (facility_id)
        REFERENCES facility(id)
        ON DELETE CASCADE,

    CONSTRAINT uq_course_facility
        UNIQUE (master_course_id, facility_id)
);
