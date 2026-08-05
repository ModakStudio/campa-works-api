use serde::{Deserialize, Serialize};

use crate::{
    dto::{major::MajorResponse, master_course::MasterCourseResponse, semester::SemesterResponse},
    models::{
        course::Course,
        enums::{CourseCategory, Language},
        major::Major,
        master_course::MasterCourse,
        semester::Semester,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateCourseRequest {
    pub master_course_id: i64,
    pub semester_id: i64,
    pub major_id: i64,

    pub description: Option<String>,

    pub course_category: CourseCategory,

    pub language: Language,

    pub section_number: i32,
    pub grade: i32,
    pub capacity: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourseRequest {
    pub description: Option<String>,

    pub course_category: Option<CourseCategory>,

    pub language: Option<Language>,

    pub section_number: Option<i32>,
    pub grade: Option<i32>,
    pub capacity: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CourseResponse {
    pub id: i64,

    pub master_course: MasterCourseResponse,
    pub semester: SemesterResponse,
    pub major: MajorResponse,

    pub description: Option<String>,

    pub course_category: CourseCategory,

    pub language: Language,

    pub section_number: i32,
    pub grade: i32,
    pub capacity: i32,
}

impl From<(Course, MasterCourse, Semester, Major)> for CourseResponse {
    fn from(
        (course, master_course, semester, major): (Course, MasterCourse, Semester, Major),
    ) -> Self {
        Self {
            id: course.id,

            master_course: MasterCourseResponse::from(master_course),
            semester: SemesterResponse::from(semester),
            major: MajorResponse::from(major),

            description: course.description,

            course_category: course.course_category,

            language: course.language,

            section_number: course.section_number,
            grade: course.grade,
            capacity: course.capacity,
        }
    }
}
