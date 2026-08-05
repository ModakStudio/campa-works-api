use diesel::prelude::*;

use crate::schema::course_preference_bookmark;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = course_preference_bookmark)]
pub struct CoursePreferenceBookmark {
    pub id: i64,

    pub professor_id: i64,
    pub master_course_id: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = course_preference_bookmark)]
pub struct NewCoursePreferenceBookmark {
    pub professor_id: i64,
    pub master_course_id: i64,
}
