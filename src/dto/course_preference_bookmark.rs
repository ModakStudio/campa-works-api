use serde::{Deserialize, Serialize};

use crate::{
    dto::{master_course::MasterCourseResponse, professor::ProfessorResponse},
    models::{
        course_preference_bookmark::CoursePreferenceBookmark, master_course::MasterCourse,
        professor::Professor, semester::Semester, user::User,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateCoursePreferenceBookmarkRequest {
    pub professor_id: i64,
    pub master_course_id: i64,
}

#[derive(Debug, Serialize)]
pub struct CoursePreferenceBookmarkResponse {
    pub id: i64,

    pub professor: ProfessorResponse,
    pub master_course: MasterCourseResponse,
}

impl
    From<(
        CoursePreferenceBookmark,
        Professor,
        User,
        Semester,
        MasterCourse,
    )> for CoursePreferenceBookmarkResponse
{
    fn from(
        (bookmark, professor, user, semester, master_course): (
            CoursePreferenceBookmark,
            Professor,
            User,
            Semester,
            MasterCourse,
        ),
    ) -> Self {
        Self {
            id: bookmark.id,

            professor: ProfessorResponse::from((professor, user, semester)),
            master_course: MasterCourseResponse::from(master_course),
        }
    }
}
