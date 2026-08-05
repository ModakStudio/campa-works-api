use diesel::prelude::*;

use crate::schema::course_facility;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = course_facility)]
pub struct CourseFacility {
    pub id: i64,

    pub master_course_id: i64,
    pub facility_id: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = course_facility)]
pub struct NewCourseFacility {
    pub master_course_id: i64,
    pub facility_id: i64,
}
