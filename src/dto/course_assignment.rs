use serde::{Deserialize, Serialize};

use crate::{
    dto::{course::CourseResponse, professor::ProfessorBriefResponse},
    models::{
        course::Course, course_assignment::CourseAssignment, course_curriculum::CourseCurriculum,
        curriculum::Curriculum, major::Major, master_course::MasterCourse, professor::Professor,
        semester::Semester, user::User,
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
    pub professor: ProfessorBriefResponse,
}

impl
    From<(
        CourseAssignment,
        Course,
        CourseCurriculum,
        MasterCourse,
        Curriculum,
        Semester,
        Major,
        Professor,
        User,
    )> for CourseAssignmentResponse
{
    fn from(
        (
            course_assignment,
            course,
            course_curriculum,
            master_course,
            curriculum,
            semester,
            major,
            professor,
            user,
        ): (
            CourseAssignment,
            Course,
            CourseCurriculum,
            MasterCourse,
            Curriculum,
            Semester,
            Major,
            Professor,
            User,
        ),
    ) -> Self {
        Self {
            id: course_assignment.id,

            course: CourseResponse::from((
                course,
                course_curriculum,
                master_course,
                curriculum,
                semester,
                major,
            )),
            professor: ProfessorBriefResponse::from((professor, user)),
        }
    }
}
