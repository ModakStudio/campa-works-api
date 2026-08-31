use serde::{Deserialize, Serialize};

use crate::models::{
    enums::{CourseStatus, CourseType},
    master_course::MasterCourse,
};

#[derive(Debug, Deserialize)]
pub struct CreateMasterCourseRequest {
    pub course_code: String,
    pub course_name: String,
    pub course_en_name: String,

    pub course_type: CourseType,

    pub is_core: bool,

    pub course_status: CourseStatus,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMasterCourseRequest {
    pub course_code: Option<String>,
    pub course_name: Option<String>,
    pub course_en_name: Option<String>,

    pub course_type: Option<CourseType>,

    pub is_core: Option<bool>,

    pub course_status: Option<CourseStatus>,
}

#[derive(Debug, Serialize)]
pub struct MasterCourseResponse {
    pub id: i64,

    pub course_code: String,
    pub course_name: String,
    pub course_en_name: String,

    pub course_type: CourseType,

    pub is_core: bool,

    pub course_status: CourseStatus,
}

impl From<MasterCourse> for MasterCourseResponse {
    fn from(course: MasterCourse) -> Self {
        Self {
            id: course.id,

            course_code: course.course_code,
            course_name: course.course_name,
            course_en_name: course.course_en_name,

            course_type: course.course_type,

            is_core: course.is_core,

            course_status: course.course_status,
        }
    }
}

impl From<&str> for CourseType {
    fn from(s: &str) -> Self {
        match s {
            "UNDERGRADUATE" => CourseType::Undergraduate,
            "GRADUATE" => CourseType::Graduate,
            _ => panic!("Invalid CourseType string: {}", s),
        }
    }
}

impl From<&str> for CourseStatus {
    fn from(s: &str) -> Self {
        match s {
            "ACTIVE" => CourseStatus::Active,
            "INACTIVE" => CourseStatus::Inactive,
            _ => panic!("Invalid CourseStatus string: {}", s),
        }
    }
}
