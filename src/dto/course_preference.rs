use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    dto::{
        master_course::MasterCourseResponse, professor::ProfessorResponse,
        semester::SemesterResponse,
    },
    models::{
        course_preference::CoursePreference, master_course::MasterCourse, professor::Professor,
        semester::Semester, user::User,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateCoursePreferenceRequest {
    pub semester_id: i64,
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

    pub semester: SemesterResponse,
    pub professor: ProfessorResponse,
    pub master_course: MasterCourseResponse,

    pub priority: i32,

    pub created_at: NaiveDateTime,
}

impl From<(CoursePreference, Semester, Professor, User, MasterCourse)>
    for CoursePreferenceResponse
{
    fn from(
        (course_preference, semester, professor, user, master_course): (
            CoursePreference,
            Semester,
            Professor,
            User,
            MasterCourse,
        ),
    ) -> Self {
        Self {
            id: course_preference.id,

            semester: SemesterResponse::from(semester),
            professor: ProfessorResponse::from((professor, user)),
            master_course: MasterCourseResponse::from(master_course),

            priority: course_preference.priority,

            created_at: course_preference.created_at,
        }
    }
}
