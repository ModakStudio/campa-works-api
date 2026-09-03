use diesel::prelude::*;

use crate::schema::course_curriculum;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = course_curriculum)]
pub struct CourseCurriculum {
    pub id: i64,

    pub master_course_id: i64,
    pub curriculum_id: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = course_curriculum)]
pub struct NewCourseCurriculum {
    pub master_course_id: i64,
    pub curriculum_id: i64,
}
