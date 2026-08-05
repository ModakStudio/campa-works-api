use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    dto::{master_course::MasterCourseResponse, professor::ProfessorResponse},
    models::{
        course_pool::CoursePool, master_course::MasterCourse, professor::Professor, user::User,
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

    pub created_at: NaiveDateTime,
}

impl From<(CoursePool, Professor, User, MasterCourse)> for CoursePoolResponse {
    fn from(
        (course_pool, professor, user, master_course): (CoursePool, Professor, User, MasterCourse),
    ) -> Self {
        Self {
            id: course_pool.id,

            professor: ProfessorResponse::from((professor, user)),

            master_course: MasterCourseResponse::from(master_course),

            created_at: course_pool.created_at,
        }
    }
}
