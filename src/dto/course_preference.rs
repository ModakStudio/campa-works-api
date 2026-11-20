use serde::{Deserialize, Serialize};

use crate::{
    dto::{course::CourseResponse, professor::ProfessorBriefResponse},
    models::{
        course::Course, course_curriculum::CourseCurriculum, course_preference::CoursePreference,
        curriculum::Curriculum, major::Major, master_course::MasterCourse, professor::Professor,
        semester::Semester, user::User,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateCoursePreferenceRequest {
    pub professor_id: i64,
    pub course_id: i64,

    pub priority: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCoursePreferenceRequest {
    pub priority: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CoursePreferenceResponse {
    pub id: i64,

    pub professor: ProfessorBriefResponse,
    pub course: CourseResponse,

    pub priority: i32,
}

impl
    From<(
        CoursePreference,
        Professor,
        User,
        Course,
        CourseCurriculum,
        MasterCourse,
        Curriculum,
        Semester,
        Major,
    )> for CoursePreferenceResponse
{
    fn from(
        (
            course_preference,
            professor,
            user,
            course,
            course_curriculum,
            master_course,
            curriculum,
            semester,
            major,
        ): (
            CoursePreference,
            Professor,
            User,
            Course,
            CourseCurriculum,
            MasterCourse,
            Curriculum,
            Semester,
            Major,
        ),
    ) -> Self {
        Self {
            id: course_preference.id,

            professor: ProfessorBriefResponse::from((professor, user)),
            course: CourseResponse::from((
                course,
                course_curriculum,
                master_course,
                curriculum,
                semester,
                major,
            )),

            priority: course_preference.priority,
        }
    }
}
