use serde::{Deserialize, Serialize};

use crate::models::{enums::CourseType, master_course::MasterCourse};

#[derive(Debug, Deserialize)]
pub struct CreateMasterCourseRequest {
    pub course_code: String,
    pub name: String,

    pub credit: i32,
    pub lecture: i32,
    pub practice: i32,

    pub course_type: CourseType,

    pub is_core: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMasterCourseRequest {
    pub course_code: Option<String>,
    pub name: Option<String>,

    pub credit: Option<i32>,
    pub lecture: Option<i32>,
    pub practice: Option<i32>,

    pub course_type: Option<CourseType>,

    pub is_core: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct MasterCourseResponse {
    pub id: i64,

    pub course_code: String,
    pub name: String,

    pub credit: i32,
    pub lecture: i32,
    pub practice: i32,

    pub course_type: CourseType,

    pub is_core: bool,
}

impl From<MasterCourse> for MasterCourseResponse {
    fn from(course: MasterCourse) -> Self {
        Self {
            id: course.id,
            course_code: course.course_code,
            name: course.name,
            credit: course.credit,
            lecture: course.lecture,
            practice: course.practice,
            course_type: course.course_type,
            is_core: course.is_core,
        }
    }
}
