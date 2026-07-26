use serde::{Deserialize, Serialize};

use crate::models::{
    enums::{ProfessorPosition, ProfessorStatus, UserRole},
    professor::Professor,
    user::User,
};

#[derive(Debug, Deserialize)]
pub struct CreateProfessorRequest {
    pub user_id: i64,

    pub position: ProfessorPosition,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfessorRequest {
    pub position: Option<ProfessorPosition>,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub status: Option<ProfessorStatus>,
}

#[derive(Debug, Serialize)]
pub struct ProfessorUserResponse {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize)]
pub struct ProfessorResponse {
    pub id: i64,

    pub user: ProfessorUserResponse,

    pub position: ProfessorPosition,

    pub office: Option<String>,
    pub tel: Option<String>,
    pub research_field: Option<String>,

    pub status: ProfessorStatus,
}

impl From<(Professor, User)> for ProfessorResponse {
    fn from((professor, user): (Professor, User)) -> Self {
        Self {
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
        }
    }
}
