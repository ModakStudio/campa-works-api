/* ============================================================
   ENUM TYPES
============================================================ */

CREATE TYPE user_role AS ENUM (
    'ADMIN',
    'PROFESSOR',
    'ASSISTANT',
    'STAFF'
);

CREATE TYPE professor_position AS ENUM (
    'PROFESSOR',
    'INVITED',
    'CONCURRENT',
    'VISITING',
    'EMERITUS'
);

CREATE TYPE professor_status AS ENUM (
    'ACTIVE',
    'INACTIVE'
);

CREATE TYPE quota_type AS ENUM (
    'CREDIT',
    'HOUR'
);

CREATE TYPE semester_type AS ENUM (
    'FIRST',
    'SUMMER',
    'SECOND',
    'WINTER'
);

CREATE TYPE semester_status AS ENUM (
    'ACTIVE',
    'INACTIVE'
);

CREATE TYPE major_status AS ENUM (
    'ACTIVE',
    'INACTIVE'
);

CREATE TYPE course_type AS ENUM (
    'UNDERGRADUATE',
    'GRADUATE'
);

CREATE TYPE course_status AS ENUM (
    'ACTIVE',
    'INACTIVE'
);

CREATE TYPE course_category AS ENUM (
    'MAJOR_REQUIRED',
    'MAJOR_ELECTIVE',
    'GENERAL_REQUIRED',
    'GENERAL_ELECTIVE'
);

CREATE TYPE language AS ENUM (
    'KOREAN',
    'ENGLISH'
);

CREATE TYPE day_of_week AS ENUM (
    'MON',
    'TUE',
    'WED',
    'THU',
    'FRI',
    'SAT',
    'SUN'
);


/* ============================================================
   USERS
============================================================ */

CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,

    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    username VARCHAR NOT NULL,

    role user_role NOT NULL,

    is_super BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


/* ============================================================
   SEMESTER
============================================================ */

CREATE TABLE semester (
    id BIGSERIAL PRIMARY KEY,

    year INTEGER NOT NULL,

    semester_ semester_type NOT NULL,

    semester_status semester_status NOT NULL
        DEFAULT 'ACTIVE',

    CONSTRAINT semester_year_semester_unique
        UNIQUE (year, semester_)
);


/* ============================================================
   MAJOR
============================================================ */

CREATE TABLE major (
    id BIGSERIAL PRIMARY KEY,

    major_name VARCHAR NOT NULL,
    major_code VARCHAR NOT NULL,

    major_status major_status NOT NULL
        DEFAULT 'ACTIVE'
);


/* ============================================================
   PROFESSOR
============================================================ */

CREATE TABLE professor (
    id BIGSERIAL PRIMARY KEY,

    user_id BIGINT NOT NULL UNIQUE,

    position professor_position NOT NULL,

    office VARCHAR,
    tel VARCHAR,
    research_field VARCHAR,

    appointed_at BIGINT NOT NULL UNIQUE,

    professor_status professor_status NOT NULL
        DEFAULT 'ACTIVE',

    CONSTRAINT professor_user_id_fkey
        FOREIGN KEY (user_id)
        REFERENCES users(id),

    CONSTRAINT professor_appointed_at_fkey
        FOREIGN KEY (appointed_at)
        REFERENCES semester(id)
);


/* ============================================================
   PROFESSOR QUOTA
============================================================ */

CREATE TABLE professor_quota (
    id BIGSERIAL PRIMARY KEY,

    professor_id BIGINT NOT NULL,
    semester_id BIGINT NOT NULL,

    quota_type quota_type NOT NULL,
    quota_value INTEGER NOT NULL,

    CONSTRAINT professor_quota_professor_semester_unique
        UNIQUE (professor_id, semester_id),

    CONSTRAINT professor_quota_professor_id_fkey
        FOREIGN KEY (professor_id)
        REFERENCES professor(id),

    CONSTRAINT professor_quota_semester_id_fkey
        FOREIGN KEY (semester_id)
        REFERENCES semester(id)
);


/* ============================================================
   CURRICULUM
============================================================ */

CREATE TABLE curriculum (
    id BIGSERIAL PRIMARY KEY,

    semester_id BIGINT NOT NULL,
    major_id BIGINT NOT NULL,

    CONSTRAINT curriculum_semester_major_unique
        UNIQUE (semester_id, major_id),

    CONSTRAINT curriculum_semester_id_fkey
        FOREIGN KEY (semester_id)
        REFERENCES semester(id),

    CONSTRAINT curriculum_major_id_fkey
        FOREIGN KEY (major_id)
        REFERENCES major(id)
);


/* ============================================================
   MASTER COURSE
============================================================ */

CREATE TABLE master_course (
    id BIGSERIAL PRIMARY KEY,

    course_code VARCHAR NOT NULL,
    course_name VARCHAR NOT NULL,
    course_en_name VARCHAR NOT NULL,

    course_type course_type NOT NULL,

    is_core BOOLEAN NOT NULL DEFAULT FALSE,

    course_status course_status NOT NULL
        DEFAULT 'ACTIVE'
);


/* ============================================================
   COURSE CURRICULUM
============================================================ */

CREATE TABLE course_curriculum (
    id BIGSERIAL PRIMARY KEY,

    master_course_id BIGINT NOT NULL,
    curriculum_id BIGINT NOT NULL,

    CONSTRAINT course_curriculum_master_course_curriculum_unique
        UNIQUE (master_course_id, curriculum_id),

    CONSTRAINT course_curriculum_master_course_id_fkey
        FOREIGN KEY (master_course_id)
        REFERENCES master_course(id),

    CONSTRAINT course_curriculum_curriculum_id_fkey
        FOREIGN KEY (curriculum_id)
        REFERENCES curriculum(id)
);


/* ============================================================
   COURSE
============================================================ */

CREATE TABLE course (
    id BIGSERIAL PRIMARY KEY,

    master_course_id BIGINT NOT NULL,

    course_description VARCHAR,

    grade INTEGER NOT NULL,
    credit INTEGER NOT NULL,
    lecture INTEGER NOT NULL,
    practice INTEGER NOT NULL,

    course_category course_category NOT NULL,

    language language NOT NULL,

    section_number INTEGER NOT NULL,
    capacity INTEGER NOT NULL,
    participant INTEGER NOT NULL DEFAULT 0,

    CONSTRAINT course_master_course_section_unique
        UNIQUE (master_course_id, section_number),

    CONSTRAINT course_master_course_id_fkey
        FOREIGN KEY (master_course_id)
        REFERENCES master_course(id)
);


/* ============================================================
   COURSE POOL
============================================================ */

CREATE TABLE course_pool (
    id BIGSERIAL PRIMARY KEY,

    professor_id BIGINT NOT NULL,
    master_course_id BIGINT NOT NULL,

    CONSTRAINT course_pool_professor_master_course_unique
        UNIQUE (professor_id, master_course_id),

    CONSTRAINT course_pool_professor_id_fkey
        FOREIGN KEY (professor_id)
        REFERENCES professor(id),

    CONSTRAINT course_pool_master_course_id_fkey
        FOREIGN KEY (master_course_id)
        REFERENCES master_course(id)
);


/* ============================================================
   COURSE PREFERENCE
============================================================ */

CREATE TABLE course_preference (
    id BIGSERIAL PRIMARY KEY,

    professor_id BIGINT NOT NULL,
    master_course_id BIGINT NOT NULL,

    priority INTEGER NOT NULL,

    CONSTRAINT course_preference_professor_priority_unique
        UNIQUE (professor_id, priority),

    CONSTRAINT course_preference_professor_master_course_unique
        UNIQUE (professor_id, master_course_id),

    CONSTRAINT course_preference_professor_id_fkey
        FOREIGN KEY (professor_id)
        REFERENCES professor(id),

    CONSTRAINT course_preference_master_course_id_fkey
        FOREIGN KEY (master_course_id)
        REFERENCES master_course(id)
);


/* ============================================================
   COURSE PREFERENCE BOOKMARK
============================================================ */

CREATE TABLE course_preference_bookmark (
    id BIGSERIAL PRIMARY KEY,

    professor_id BIGINT NOT NULL,
    master_course_id BIGINT NOT NULL,

    CONSTRAINT course_preference_bookmark_professor_master_course_unique
        UNIQUE (professor_id, master_course_id),

    CONSTRAINT course_preference_bookmark_professor_id_fkey
        FOREIGN KEY (professor_id)
        REFERENCES professor(id),

    CONSTRAINT course_preference_bookmark_master_course_id_fkey
        FOREIGN KEY (master_course_id)
        REFERENCES master_course(id)
);


/* ============================================================
   COURSE ASSIGNMENT
============================================================ */

CREATE TABLE course_assignment (
    id BIGSERIAL PRIMARY KEY,

    course_id BIGINT NOT NULL,
    professor_id BIGINT NOT NULL,

    CONSTRAINT course_assignment_course_professor_unique
        UNIQUE (course_id, professor_id),

    CONSTRAINT course_assignment_course_id_fkey
        FOREIGN KEY (course_id)
        REFERENCES course(id),

    CONSTRAINT course_assignment_professor_id_fkey
        FOREIGN KEY (professor_id)
        REFERENCES professor(id)
);


/* ============================================================
   CLASSROOM
============================================================ */

CREATE TABLE classroom (
    id BIGSERIAL PRIMARY KEY,

    building VARCHAR NOT NULL,
    room VARCHAR NOT NULL,

    capacity INTEGER NOT NULL,

    is_available BOOLEAN NOT NULL DEFAULT TRUE,

    CONSTRAINT classroom_building_room_unique
        UNIQUE (building, room)
);


/* ============================================================
   FACILITY
============================================================ */

CREATE TABLE facility (
    id BIGSERIAL PRIMARY KEY,

    facility_name VARCHAR NOT NULL UNIQUE,

    facility_description VARCHAR
);


/* ============================================================
   CLASSROOM FACILITY
============================================================ */

CREATE TABLE classroom_facility (
    id BIGSERIAL PRIMARY KEY,

    classroom_id BIGINT NOT NULL,
    facility_id BIGINT NOT NULL,

    CONSTRAINT classroom_facility_classroom_facility_unique
        UNIQUE (classroom_id, facility_id),

    CONSTRAINT classroom_facility_classroom_id_fkey
        FOREIGN KEY (classroom_id)
        REFERENCES classroom(id),

    CONSTRAINT classroom_facility_facility_id_fkey
        FOREIGN KEY (facility_id)
        REFERENCES facility(id)
);


/* ============================================================
   COURSE FACILITY
============================================================ */

CREATE TABLE course_facility (
    id BIGSERIAL PRIMARY KEY,

    master_course_id BIGINT NOT NULL,
    facility_id BIGINT NOT NULL,

    CONSTRAINT course_facility_master_course_facility_unique
        UNIQUE (master_course_id, facility_id),

    CONSTRAINT course_facility_master_course_id_fkey
        FOREIGN KEY (master_course_id)
        REFERENCES master_course(id),

    CONSTRAINT course_facility_facility_id_fkey
        FOREIGN KEY (facility_id)
        REFERENCES facility(id)
);


/* ============================================================
   TIMETABLE
============================================================ */

CREATE TABLE timetable (
    id BIGSERIAL PRIMARY KEY,

    assignment_id BIGINT NOT NULL,
    classroom_id BIGINT NOT NULL,

    day_of_week day_of_week NOT NULL,

    start_time TIME NOT NULL,
    end_time TIME NOT NULL,

    CONSTRAINT timetable_start_end_period_check
        CHECK (start_time <= end_time),

    CONSTRAINT timetable_assignment_id_fkey
        FOREIGN KEY (assignment_id)
        REFERENCES course_assignment(id),

    CONSTRAINT timetable_classroom_id_fkey
        FOREIGN KEY (classroom_id)
        REFERENCES classroom(id)
);
