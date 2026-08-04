use diesel::prelude::*;

use crate::{models::enums::CourseType, schema::master_course};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = master_course)]
pub struct MasterCourse {
    pub id: i64,

    pub course_code: String,
    pub name: String,

    pub credit: i32,
    pub lecture: i32,
    pub practice: i32,

    pub course_type: CourseType,

    pub is_core: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = master_course)]
pub struct NewMasterCourse {
    pub course_code: String,
    pub name: String,

    pub credit: i32,
    pub lecture: i32,
    pub practice: i32,

    pub course_type: CourseType,

    pub is_core: bool,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = master_course)]
pub struct UpdateMasterCourse {
    pub course_code: Option<String>,
    pub name: Option<String>,

    pub credit: Option<i32>,
    pub lecture: Option<i32>,
    pub practice: Option<i32>,

    pub course_type: Option<CourseType>,

    pub is_core: Option<bool>,
}
