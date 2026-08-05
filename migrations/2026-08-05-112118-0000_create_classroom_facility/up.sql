CREATE TABLE classroom_facility (
    id BIGSERIAL PRIMARY KEY,

    classroom_id BIGINT NOT NULL,
    facility_id BIGINT NOT NULL,

    CONSTRAINT fk_classroom_facility_classroom
        FOREIGN KEY (classroom_id)
        REFERENCES classroom(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_classroom_facility_facility
        FOREIGN KEY (facility_id)
        REFERENCES facility(id)
        ON DELETE CASCADE,

    CONSTRAINT uq_classroom_facility
        UNIQUE (classroom_id, facility_id)
);
