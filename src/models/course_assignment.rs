use diesel::prelude::*;

use crate::schema::course_assignment;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = course_assignment)]
pub struct CourseAssignment {
    pub id: i64,

    pub course_id: i64,
    pub professor_id: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = course_assignment)]
pub struct NewCourseAssignment {
    pub course_id: i64,
    pub professor_id: i64,
}
