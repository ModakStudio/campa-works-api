use serde::{Deserialize, Serialize};

use crate::{
    dto::{master_course::MasterCourseResponse, professor::ProfessorResponse},
    models::{
        course_preference::CoursePreference, master_course::MasterCourse, professor::Professor,
        semester::Semester, user::User,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateCoursePreferenceRequest {
    pub professor_id: i64,
    pub master_course_id: i64,

    pub priority: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCoursePreferenceRequest {
    pub priority: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CoursePreferenceResponse {
    pub id: i64,

    pub professor: ProfessorResponse,
    pub master_course: MasterCourseResponse,

    pub priority: i32,
}

impl From<(CoursePreference, Professor, User, Semester, MasterCourse)>
    for CoursePreferenceResponse
{
    fn from(
        (course_preference, professor, user, semester, master_course): (
            CoursePreference,
            Professor,
            User,
            Semester,
            MasterCourse,
        ),
    ) -> Self {
        Self {
            id: course_preference.id,

            professor: ProfessorResponse::from((professor, user, semester)),
            master_course: MasterCourseResponse::from(master_course),

            priority: course_preference.priority,
        }
    }
}
