CREATE TABLE facility (
    id BIGSERIAL PRIMARY KEY,

    name VARCHAR NOT NULL UNIQUE,
    description VARCHAR
);
