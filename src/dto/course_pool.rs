use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    dto::professor::ProfessorUserResponse,
    models::{
        course_pool::CoursePool,
        enums::{CourseType, ProfessorPosition, ProfessorStatus},
        master_course::MasterCourse,
        professor::Professor,
        user::User,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateCoursePoolRequest {
    pub professor_id: i64,
    pub master_course_id: i64,
}

#[derive(Debug, Serialize)]
pub struct CoursePoolProfessorResponse {
    pub id: i64,

    pub user: ProfessorUserResponse,

    pub position: ProfessorPosition,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub status: ProfessorStatus,
}

#[derive(Debug, Serialize)]
pub struct CoursePoolMasterCourseResponse {
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
pub struct CoursePoolResponse {
    pub id: i64,

    pub professor: CoursePoolProfessorResponse,

    pub master_course: CoursePoolMasterCourseResponse,

    pub created_at: NaiveDateTime,
}

impl From<(CoursePool, Professor, User, MasterCourse)> for CoursePoolResponse {
    fn from(
        (course_pool, professor, user, master_course): (CoursePool, Professor, User, MasterCourse),
    ) -> Self {
        Self {
            id: course_pool.id,

            professor: CoursePoolProfessorResponse {
                id: professor.id,

                user: ProfessorUserResponse {
                    id: user.id,
                    email: user.email,
                    name: user.name,
                    role: user.role,
                },

                position: professor.position,

                office: professor.office,
                tel: professor.tel,
                research_field: professor.research_field,

                status: professor.status,
            },

            master_course: CoursePoolMasterCourseResponse {
                id: master_course.id,

                course_code: master_course.course_code,
                name: master_course.name,

                credit: master_course.credit,
                lecture: master_course.lecture,
                practice: master_course.practice,

                course_type: master_course.course_type,

                is_core: master_course.is_core,
            },

            created_at: course_pool.created_at,
        }
    }
}
