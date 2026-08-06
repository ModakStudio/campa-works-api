CREATE TABLE timetable (
    id BIGSERIAL PRIMARY KEY,

    assignment_id BIGINT NOT NULL,
    classroom_id BIGINT NOT NULL,

    day_of_week INT NOT NULL,

    start_period INT NOT NULL,
    end_period INT NOT NULL,

    CONSTRAINT fk_timetable_assignment
        FOREIGN KEY (assignment_id)
        REFERENCES course_assignment(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_timetable_classroom
        FOREIGN KEY (classroom_id)
        REFERENCES classroom(id)
        ON DELETE CASCADE,

    CONSTRAINT chk_timetable_period
        CHECK (
            start_period >= 1
            AND end_period >= start_period
        )
);
