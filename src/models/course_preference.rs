use diesel::prelude::*;

use crate::schema::course_preference;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = course_preference)]
pub struct CoursePreference {
    pub id: i64,

    pub semester_id: i64,
    pub professor_id: i64,
    pub master_course_id: i64,

    pub priority: i32,

    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = course_preference)]
pub struct NewCoursePreference {
    pub semester_id: i64,
    pub professor_id: i64,
    pub master_course_id: i64,

    pub priority: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = course_preference)]
pub struct UpdateCoursePreference {
    pub priority: Option<i32>,
}
