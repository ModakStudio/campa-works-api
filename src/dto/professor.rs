use serde::{Deserialize, Serialize};

use crate::{
    dto::{semester::SemesterResponse, user::UserResponse},
    models::{
        enums::{ProfessorPosition, ProfessorStatus},
        professor::Professor,
        semester::Semester,
        user::User,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateProfessorRequest {
    pub user_id: i64,

    pub position: ProfessorPosition,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub appointed_at: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfessorRequest {
    pub position: Option<ProfessorPosition>,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub appointed_at: Option<i64>,

    pub professor_status: Option<ProfessorStatus>,
}

#[derive(Debug, Serialize)]
pub struct ProfessorResponse {
    pub id: i64,

    pub user: UserResponse,

    pub position: ProfessorPosition,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub appointed_at: SemesterResponse,

    pub professor_status: ProfessorStatus,
}

#[derive(Debug, Serialize)]
pub struct ProfessorBriefResponse {
    pub id: i64,

    pub user: UserResponse,

    pub position: ProfessorPosition,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub professor_status: ProfessorStatus,
}

impl From<(Professor, User, Semester)> for ProfessorResponse {
    fn from((professor, user, semester): (Professor, User, Semester)) -> Self {
        Self {
            id: professor.id,

            user: UserResponse::from(user),

            position: professor.position,

            office: professor.office,
            tel: professor.tel,
            research_field: professor.research_field,

            appointed_at: SemesterResponse::from(semester),

            professor_status: professor.professor_status,
        }
    }
}

impl From<(Professor, User)> for ProfessorBriefResponse {
    fn from((professor, user): (Professor, User)) -> Self {
        Self {
            id: professor.id,

            user: UserResponse::from(user),

            position: professor.position,

            office: professor.office,
            tel: professor.tel,
            research_field: professor.research_field,

            professor_status: professor.professor_status,
        }
    }
}

impl From<&str> for ProfessorPosition {
    fn from(s: &str) -> Self {
        match s {
            "PROFESSOR" => ProfessorPosition::Professor,
            "INVITED" => ProfessorPosition::Invited,
            "CONCURRENT" => ProfessorPosition::Concurrent,
            "VISITING" => ProfessorPosition::Visiting,
            "EMERITUS" => ProfessorPosition::Emeritus,
            _ => ProfessorPosition::Emeritus,
        }
    }
}

impl From<&str> for ProfessorStatus {
    fn from(s: &str) -> Self {
        match s {
            "ACTIVE" => ProfessorStatus::Active,
            "INACTIVE" => ProfessorStatus::Inactive,
            _ => ProfessorStatus::Inactive,
        }
    }
}
