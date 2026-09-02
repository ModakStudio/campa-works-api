use serde::{Deserialize, Serialize};

use crate::{
    dto::{course::CourseResponse, professor::ProfessorResponse},
    models::{
        course::Course, course_assignment::CourseAssignment, master_course::MasterCourse,
        professor::Professor, semester::Semester, user::User,
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
}

impl
    From<(
        CourseAssignment,
        Course,
        MasterCourse,
        Professor,
        User,
        Semester,
    )> for CourseAssignmentResponse
{
    fn from(
        (course_assignment, course, master_course, professor, user, semester): (
            CourseAssignment,
            Course,
            MasterCourse,
            Professor,
            User,
            Semester,
        ),
    ) -> Self {
        Self {
            id: course_assignment.id,

            course: CourseResponse::from((course, master_course)),
            professor: ProfessorResponse::from((professor, user, semester)),
        }
    }
}
