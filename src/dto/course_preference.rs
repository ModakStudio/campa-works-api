use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    dto::professor::ProfessorUserResponse,
    models::{
        course_preference::CoursePreference,
        enums::{CourseType, ProfessorPosition, ProfessorStatus, SemesterStatus, SemesterType},
        master_course::MasterCourse,
        professor::Professor,
        semester::Semester,
        user::User,
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
pub struct CoursePreferenceSemesterResponse {
    pub id: i64,

    pub year: i32,
    pub semester_: SemesterType,

    pub status: SemesterStatus,
}

#[derive(Debug, Serialize)]
pub struct CoursePreferenceProfessorResponse {
    pub id: i64,

    pub user: ProfessorUserResponse,

    pub position: ProfessorPosition,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub status: ProfessorStatus,
}

#[derive(Debug, Serialize)]
pub struct CoursePreferenceMasterCourseResponse {
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
pub struct CoursePreferenceResponse {
    pub id: i64,

    pub semester: CoursePreferenceSemesterResponse,
    pub professor: CoursePreferenceProfessorResponse,
    pub master_course: CoursePreferenceMasterCourseResponse,

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

            semester: CoursePreferenceSemesterResponse {
                id: semester.id,
                year: semester.year,
                semester_: semester.semester_,
                status: semester.status,
            },

            professor: CoursePreferenceProfessorResponse {
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

            master_course: CoursePreferenceMasterCourseResponse {
                id: master_course.id,

                course_code: master_course.course_code,
                name: master_course.name,

                credit: master_course.credit,
                lecture: master_course.lecture,
                practice: master_course.practice,

                course_type: master_course.course_type,

                is_core: master_course.is_core,
            },

            priority: course_preference.priority,

            created_at: course_preference.created_at,
        }
    }
}
