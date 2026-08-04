use serde::{Deserialize, Serialize};

use crate::models::{
    course::Course,
    enums::{CourseCategory, CourseType, Language, SemesterStatus, SemesterType},
    major::Major,
    master_course::MasterCourse,
    semester::Semester,
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
pub struct CourseMasterCourseResponse {
    pub id: i64,

    pub course_code: String,
    pub name: String,

    pub credit: i32,
    pub lecture: i32,
    pub practice: i32,

    pub course_type: CourseType,

    pub is_core: bool,
}

#[derive(Debug, Serialize)]
pub struct CourseSemesterResponse {
    pub id: i64,

    pub year: i32,
    pub semester_: SemesterType,

    pub status: SemesterStatus,
}

#[derive(Debug, Serialize)]
pub struct CourseMajorResponse {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CourseResponse {
    pub id: i64,

    pub master_course: CourseMasterCourseResponse,
    pub semester: CourseSemesterResponse,
    pub major: CourseMajorResponse,

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

            master_course: CourseMasterCourseResponse {
                id: master_course.id,

                course_code: master_course.course_code,
                name: master_course.name,

                credit: master_course.credit,
                lecture: master_course.lecture,
                practice: master_course.practice,

                course_type: master_course.course_type,

                is_core: master_course.is_core,
            },

            semester: CourseSemesterResponse {
                id: semester.id,

                year: semester.year,
                semester_: semester.semester_,

                status: semester.status,
            },

            major: CourseMajorResponse {
                id: major.id,
                name: major.name,
            },

            description: course.description,

            course_category: course.course_category,

            language: course.language,

            section_number: course.section_number,
            grade: course.grade,
            capacity: course.capacity,
        }
    }
}
