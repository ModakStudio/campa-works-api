use serde::{Deserialize, Serialize};

use crate::{
    dto::master_course::MasterCourseResponse,
    models::{
        course::Course,
        enums::{CourseCategory, Language},
        master_course::MasterCourse,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateCourseRequest {
    pub master_course_id: i64,

    pub course_description: Option<String>,

    pub grade: i32,
    pub credit: i32,
    pub lecture: i32,
    pub practice: i32,

    pub course_category: CourseCategory,

    pub language: Language,

    pub section_number: i32,
    pub capacity: i32,
    pub participant: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourseRequest {
    pub course_description: Option<String>,

    pub grade: Option<i32>,
    pub credit: Option<i32>,
    pub lecture: Option<i32>,
    pub practice: Option<i32>,

    pub course_category: Option<CourseCategory>,

    pub language: Option<Language>,

    pub section_number: Option<i32>,
    pub capacity: Option<i32>,
    pub participant: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CourseResponse {
    pub id: i64,

    pub master_course: MasterCourseResponse,

    pub course_description: Option<String>,

    pub grade: i32,
    pub credit: i32,
    pub lecture: i32,
    pub practice: i32,

    pub course_category: CourseCategory,

    pub language: Language,

    pub section_number: i32,
    pub capacity: i32,
    pub participant: i32,
}

impl From<(Course, MasterCourse)> for CourseResponse {
    fn from((course, master_course): (Course, MasterCourse)) -> Self {
        Self {
            id: course.id,

            master_course: MasterCourseResponse::from(master_course),

            course_description: course.course_description,

            grade: course.grade,
            credit: course.credit,
            lecture: course.lecture,
            practice: course.practice,

            course_category: course.course_category,

            language: course.language,

            section_number: course.section_number,
            capacity: course.capacity,
            participant: course.participant,
        }
    }
}

impl From<&str> for CourseCategory {
    fn from(value: &str) -> Self {
        match value {
            "MAJOR_REQUIRED" => CourseCategory::MajorRequired,
            "MAJOR_ELECTIVE" => CourseCategory::MajorElective,
            "GENERAL_REQUIRED" => CourseCategory::GeneralRequired,
            "GENERAL_ELECTIVE" => CourseCategory::GeneralElective,
            _ => panic!("Invalid course category"),
        }
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value {
            "KOREAN" => Language::Korean,
            "ENGLISH" => Language::English,
            _ => panic!("Invalid language"),
        }
    }
}
