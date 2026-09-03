use serde::{Deserialize, Serialize};

use crate::{
    dto::{master_course::MasterCourseResponse, professor::ProfessorResponse},
    models::{
        course_pool::CoursePool, master_course::MasterCourse, professor::Professor,
        semester::Semester, user::User,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateCoursePoolRequest {
    pub professor_id: i64,
    pub master_course_id: i64,
}

#[derive(Debug, Serialize)]
pub struct CoursePoolResponse {
    pub id: i64,

    pub professor: ProfessorResponse,
    pub master_course: MasterCourseResponse,
}

impl From<(CoursePool, Professor, User, Semester, MasterCourse)> for CoursePoolResponse {
    fn from(
        (course_pool, professor, user, semester, master_course): (
            CoursePool,
            Professor,
            User,
            Semester,
            MasterCourse,
        ),
    ) -> Self {
        Self {
            id: course_pool.id,

            professor: ProfessorResponse::from((professor, user, semester)),
            master_course: MasterCourseResponse::from(master_course),
        }
    }
}
