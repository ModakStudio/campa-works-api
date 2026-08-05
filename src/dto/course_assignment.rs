use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    dto::{course::CourseResponse, professor::ProfessorResponse},
    models::{
        course::Course, course_assignment::CourseAssignment, major::Major,
        master_course::MasterCourse, professor::Professor, semester::Semester, user::User,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateCourseAssignmentRequest {
    pub course_id: i64,
    pub professor_id: i64,
}

#[derive(Debug, Serialize)]
pub struct CourseAssignmentResponse {
    pub id: i64,

    pub course: CourseResponse,

    pub professor: ProfessorResponse,

    pub created_at: NaiveDateTime,
}

impl
    From<(
        CourseAssignment,
        Course,
        MasterCourse,
        Semester,
        Major,
        Professor,
        User,
    )> for CourseAssignmentResponse
{
    fn from(
        (course_assignment, course, master_course, semester, major, professor, user): (
            CourseAssignment,
            Course,
            MasterCourse,
            Semester,
            Major,
            Professor,
            User,
        ),
    ) -> Self {
        Self {
            id: course_assignment.id,

            course: CourseResponse::from((course, master_course, semester, major)),

            professor: ProfessorResponse::from((professor, user)),

            created_at: course_assignment.created_at,
        }
    }
}
