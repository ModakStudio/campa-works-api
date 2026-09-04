// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "course_category"))]
    pub struct CourseCategory;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "course_status"))]
    pub struct CourseStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "course_type"))]
    pub struct CourseType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "day_of_week"))]
    pub struct DayOfWeek;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "language"))]
    pub struct Language;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "major_status"))]
    pub struct MajorStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "professor_position"))]
    pub struct ProfessorPosition;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "professor_status"))]
    pub struct ProfessorStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "quota_type"))]
    pub struct QuotaType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "semester_status"))]
    pub struct SemesterStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "semester_type"))]
    pub struct SemesterType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    classroom (id) {
        id -> Int8,
        building -> Varchar,
        room -> Varchar,
        capacity -> Int4,
        is_available -> Bool,
    }
}

diesel::table! {
    classroom_facility (id) {
        id -> Int8,
        classroom_id -> Int8,
        facility_id -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CourseCategory;
    use super::sql_types::Language;

    course (id) {
        id -> Int8,
        master_course_id -> Int8,
        course_description -> Nullable<Varchar>,
        grade -> Int4,
        credit -> Int4,
        lecture -> Int4,
        practice -> Int4,
        course_category -> CourseCategory,
        language -> Language,
        section_number -> Int4,
        capacity -> Int4,
        participant -> Int4,
    }
}

diesel::table! {
    course_assignment (id) {
        id -> Int8,
        course_id -> Int8,
        professor_id -> Int8,
    }
}

diesel::table! {
    course_curriculum (id) {
        id -> Int8,
        master_course_id -> Int8,
        curriculum_id -> Int8,
    }
}

diesel::table! {
    course_facility (id) {
        id -> Int8,
        master_course_id -> Int8,
        facility_id -> Int8,
    }
}

diesel::table! {
    course_pool (id) {
        id -> Int8,
        professor_id -> Int8,
        master_course_id -> Int8,
    }
}

diesel::table! {
    course_preference (id) {
        id -> Int8,
        professor_id -> Int8,
        master_course_id -> Int8,
        priority -> Int4,
    }
}

diesel::table! {
    course_preference_bookmark (id) {
        id -> Int8,
        professor_id -> Int8,
        master_course_id -> Int8,
    }
}

diesel::table! {
    curriculum (id) {
        id -> Int8,
        semester_id -> Int8,
        major_id -> Int8,
    }
}

diesel::table! {
    facility (id) {
        id -> Int8,
        facility_name -> Varchar,
        facility_description -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MajorStatus;

    major (id) {
        id -> Int8,
        major_name -> Varchar,
        major_code -> Varchar,
        major_status -> MajorStatus,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CourseType;
    use super::sql_types::CourseStatus;

    master_course (id) {
        id -> Int8,
        course_code -> Varchar,
        course_name -> Varchar,
        course_en_name -> Varchar,
        course_type -> CourseType,
        is_core -> Bool,
        course_status -> CourseStatus,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProfessorPosition;
    use super::sql_types::ProfessorStatus;

    professor (id) {
        id -> Int8,
        user_id -> Int8,
        position -> ProfessorPosition,
        office -> Nullable<Varchar>,
        tel -> Nullable<Varchar>,
        research_field -> Nullable<Varchar>,
        appointed_at -> Int8,
        professor_status -> ProfessorStatus,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::QuotaType;

    professor_quota (id) {
        id -> Int8,
        professor_id -> Int8,
        semester_id -> Int8,
        quota_type -> QuotaType,
        quota_value -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SemesterType;
    use super::sql_types::SemesterStatus;

    semester (id) {
        id -> Int8,
        year -> Int4,
        semester_ -> SemesterType,
        semester_status -> SemesterStatus,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DayOfWeek;

    timetable (id) {
        id -> Int8,
        assignment_id -> Int8,
        classroom_id -> Int8,
        day_of_week -> DayOfWeek,
        start_time -> Time,
        end_time -> Time,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Int8,
        email -> Varchar,
        password -> Varchar,
        username -> Varchar,
        role -> UserRole,
        is_super -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(classroom_facility -> classroom (classroom_id));
diesel::joinable!(classroom_facility -> facility (facility_id));
diesel::joinable!(course -> master_course (master_course_id));
diesel::joinable!(course_assignment -> course (course_id));
diesel::joinable!(course_assignment -> professor (professor_id));
diesel::joinable!(course_curriculum -> curriculum (curriculum_id));
diesel::joinable!(course_curriculum -> master_course (master_course_id));
diesel::joinable!(course_facility -> facility (facility_id));
diesel::joinable!(course_facility -> master_course (master_course_id));
diesel::joinable!(course_pool -> master_course (master_course_id));
diesel::joinable!(course_pool -> professor (professor_id));
diesel::joinable!(course_preference -> master_course (master_course_id));
diesel::joinable!(course_preference -> professor (professor_id));
diesel::joinable!(course_preference_bookmark -> master_course (master_course_id));
diesel::joinable!(course_preference_bookmark -> professor (professor_id));
diesel::joinable!(curriculum -> major (major_id));
diesel::joinable!(curriculum -> semester (semester_id));
diesel::joinable!(professor -> semester (appointed_at));
diesel::joinable!(professor -> users (user_id));
diesel::joinable!(professor_quota -> professor (professor_id));
diesel::joinable!(professor_quota -> semester (semester_id));
diesel::joinable!(timetable -> classroom (classroom_id));
diesel::joinable!(timetable -> course_assignment (assignment_id));

diesel::allow_tables_to_appear_in_same_query!(
    classroom,
    classroom_facility,
    course,
    course_assignment,
    course_curriculum,
    course_facility,
    course_pool,
    course_preference,
    course_preference_bookmark,
    curriculum,
    facility,
    major,
    master_course,
    professor,
    professor_quota,
    semester,
    timetable,
    users,
);
