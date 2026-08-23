/* ============================================================
   DROP TABLES
============================================================ */

DROP TABLE IF EXISTS timetable;

DROP TABLE IF EXISTS course_facility;
DROP TABLE IF EXISTS classroom_facility;

DROP TABLE IF EXISTS facility;
DROP TABLE IF EXISTS classroom;

DROP TABLE IF EXISTS course_assignment;

DROP TABLE IF EXISTS course_preference_bookmark;
DROP TABLE IF EXISTS course_preference;
DROP TABLE IF EXISTS course_pool;

DROP TABLE IF EXISTS course;

DROP TABLE IF EXISTS course_curriculum;
DROP TABLE IF EXISTS master_course;
DROP TABLE IF EXISTS curriculum;

DROP TABLE IF EXISTS professor_quota;
DROP TABLE IF EXISTS professor;

DROP TABLE IF EXISTS major;
DROP TABLE IF EXISTS semester;

DROP TABLE IF EXISTS users;


/* ============================================================
   DROP ENUM TYPES
============================================================ */

DROP TYPE IF EXISTS day_of_week;

DROP TYPE IF EXISTS language;
DROP TYPE IF EXISTS course_category;
DROP TYPE IF EXISTS course_status;
DROP TYPE IF EXISTS course_type;

DROP TYPE IF EXISTS major_status;

DROP TYPE IF EXISTS semester_status;
DROP TYPE IF EXISTS semester_type;

DROP TYPE IF EXISTS quota_type;

DROP TYPE IF EXISTS professor_status;
DROP TYPE IF EXISTS professor_position;

DROP TYPE IF EXISTS user_role;
