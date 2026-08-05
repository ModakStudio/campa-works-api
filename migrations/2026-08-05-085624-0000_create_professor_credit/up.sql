CREATE TABLE professor_credit (
    id BIGSERIAL PRIMARY KEY,

    professor_id BIGINT NOT NULL,
    semester_id BIGINT NOT NULL,

    target_credit INT NOT NULL,

    CONSTRAINT fk_professor_credit_professor
        FOREIGN KEY (professor_id)
        REFERENCES professor(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_professor_credit_semester
        FOREIGN KEY (semester_id)
        REFERENCES semester(id)
        ON DELETE CASCADE,

    CONSTRAINT uq_professor_credit
        UNIQUE (professor_id, semester_id)
);
