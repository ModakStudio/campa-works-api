use diesel::prelude::*;

use crate::schema::course_pool;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = course_pool)]
pub struct CoursePool {
    pub id: i64,

    pub professor_id: i64,
    pub master_course_id: i64,

    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = course_pool)]
pub struct NewCoursePool {
    pub professor_id: i64,
    pub master_course_id: i64,
}
