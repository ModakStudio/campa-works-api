use serde::{Deserialize, Serialize};

use crate::{
    dto::user::UserResponse,
    models::{
        enums::{ProfessorPosition, ProfessorStatus},
        professor::Professor,
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
pub struct ProfessorResponse {
    pub id: i64,

    pub user: UserResponse,

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

            user: UserResponse::from(user),

            position: professor.position,

            office: professor.office,
            tel: professor.tel,
            research_field: professor.research_field,

            status: professor.status,
        }
    }
}
