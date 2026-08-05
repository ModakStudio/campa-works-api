CREATE TABLE classroom (
    id BIGSERIAL PRIMARY KEY,

    building VARCHAR NOT NULL,
    room VARCHAR NOT NULL,

    capacity INT NOT NULL,

    is_available BOOLEAN NOT NULL DEFAULT TRUE,

    CONSTRAINT uq_classroom
        UNIQUE (building, room)
);
