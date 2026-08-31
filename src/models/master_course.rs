use diesel::prelude::*;

use crate::{
    models::enums::{CourseStatus, CourseType},
    schema::master_course,
};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = master_course)]
pub struct MasterCourse {
    pub id: i64,

    pub course_code: String,
    pub course_name: String,
    pub course_en_name: String,

    pub course_type: CourseType,

    pub is_core: bool,

    pub course_status: CourseStatus,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = master_course)]
pub struct NewMasterCourse {
    pub course_code: String,
    pub course_name: String,
    pub course_en_name: String,

    pub course_type: CourseType,

    pub is_core: bool,

    pub course_status: CourseStatus,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = master_course)]
pub struct UpdateMasterCourse {
    pub course_code: Option<String>,
    pub course_name: Option<String>,
    pub course_en_name: Option<String>,

    pub course_type: Option<CourseType>,

    pub is_core: Option<bool>,

    pub course_status: Option<CourseStatus>,
}
